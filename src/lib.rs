#![feature(proc_macro, drain_filter)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::*;
use syn::visit_mut::VisitMut;

/// Defines a test case consisting of alternative sections.
#[proc_macro_attribute]
pub fn catch_test(_attr: TokenStream, input: TokenStream) -> TokenStream {

    let test_fn : ItemFn = parse2( input.into() ).unwrap();

    // Queue the original function and process that into results.
    // The processing takes a single fn from the queue and expands this.
    //
    // Either placing the results back into the queue for further expansion
    // or puts it into the results if no more expansions are needed.
    let mut queue = vec![ ( test_fn.ident.to_string(), test_fn ) ];
    let mut results = vec![];
    while queue.len() > 0 {

        let ( name, next ) = queue.pop().unwrap();
        let expanded = expand( &name, &next );

        // The first expansion is always without any sections.
        let mut iter = expanded.into_iter();
        let ( name, mut stripped ) = iter.next().unwrap();
        stripped.ident = Ident::new( &name, Span::call_site() );
        results.push( stripped );

        // The remaining items may contain further sections so put them into
        // the queue.
        queue.extend( iter );
    }

    quote!( #( #[test] #results )* ).into()
}

/// Expands the function into section-variants.
///
/// This includes the first variant where all sections have been removed as
/// well as the following variants where only one of the sections exists.
fn expand( name: &str, f : &ItemFn ) -> Vec<( String, ItemFn )> {

    // Section -1 will skip all sections. Start with that.
    let mut section_idx = -1;

    let mut results = vec![];
    loop {

        // Strip the sections.
        let mut next = f.clone();
        let mut visitor = PreserveNthSection( section_idx, format!( "{}", section_idx ) );
        visitor.visit_item_fn_mut( &mut next );

        // Generate the name by concatenating the initial name to the section
        // name. Or use the initial name alone if this is the first section.
        let result_name = if section_idx >= 0 {
                            format!( "{}_{}", name, visitor.1 )
                        } else {
                            name.to_string()
                        };

        results.push(( result_name, next ));

        // -1 means the last section was processed so we can break out.
        if visitor.0 == -1 { break; }

        // Keep the next section on the next iteration.
        section_idx += 1;
    }

    results
}

struct PreserveNthSection( pub i32, pub String );
impl VisitMut for PreserveNthSection {

    fn visit_expr_block_mut( &mut self, node: &mut ExprBlock ) {

        if let Some( attr ) = section_attr( &node.attrs ) {

            // Se encountered a section block. See whether this is kept or
            // stripped away.
            if self.0 != 0 {

                // Not preserved. Replace the statements with an empty list.
                node.block.stmts = vec![];

            } else {

                // Preserved. Find the name from the section but other than that
                // keep it intact.

                // ;_;
                if let Some( Meta::List( ml ) ) = attr.interpret_meta() {
                    if let Some( pair ) = ml.nested.first() {
                        if let &&NestedMeta::Literal( Lit::Str( ref s ) ) = pair.value() {
                            self.1 = sanitize_name( s.value() );
                        }
                    }
                }
            }

            // Remove the section attribute from the block now that we've
            // processed it and keep track of which section we are processing
            // next.
            node.attrs.drain_filter( |a| is_section_attribute(a) ).count();
            self.0 -= 1;

        } else {

            // Continue visiting.
            for attr in &mut node.attrs { self.visit_attribute_mut(attr) };
            self.visit_block_mut( &mut node.block );
        }
    }
}

/// Converts "the section name" into "the_section_name" suitable for fn name.
fn sanitize_name( s : String ) -> String {
    s.replace( " ", "_" ).replace( "'", "" )
}

/// Finds the #[section] attribute from the attribute list.
fn section_attr( attrs : &[Attribute] ) -> Option<Attribute> {
    attrs.iter().find( |a| is_section_attribute(a) ).cloned()
}

/// Checks whether the attribute is a #[section] attriute.
fn is_section_attribute( attr : &Attribute ) -> bool {
    attr.path.segments.last().unwrap().value().ident == "section"
}
