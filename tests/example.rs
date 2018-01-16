#![feature(proc_macro)]
extern crate ratcc;
use ratcc::*;

#[catch_test]
fn example() {

    let owned_type = String::from("foo");
    assert_eq!( owned_type, "foo" );

    #[section("first assert succeeds")] {

        let moved_here = owned_type;
        assert_eq!( moved_here, "foo" );

        #[section("second assert fails")] { assert!( false ); }
    }
    #[section("first assert fails")] {

        let moved_also_here = owned_type;
        assert_eq!( moved_also_here, "bar" );

        #[section("second assert succeeds")] {

            // The parent section fails assert so this never gets executed.
            // Should result in undefined result.
            assert!( true );
        }
    }
}

// OUTPUT:
//
//   Running target/debug/deps/test-d4d1fd68ce8fee8b
//
//   running 5 tests
//   test example ... ok
//   test example_first_assert_fails ... FAILED
//   test example_first_assert_fails_second_assert_succeeds ... FAILED
//   test example_first_assert_succeeds ... ok
//   test example_first_assert_succeeds_second_assert_fails ... FAILED
// 
