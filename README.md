# Ratcc
### Rust Automated Test Cases in a Crate

[![Crates.io](https://img.shields.io/crates/v/ratcc.svg)](https://crates.io/crates/ratcc)

A test frameworked for Rust, inspired by the [Catch] C++ test framework.

[Catch]: https://github.com/catchorg/Catch2

## Usage

```toml
[dev-dependencies]
ratcc = "0.1"
```

```rust
#![feature(proc_macro)]
extern crate ratcc;
use ratcc::*;

#[catch_test]
fn example() {

    let a = 0;
    assert_eq!(a, 0);

    #[section("first succeeds")] {
        let b = 0;
        assert_eq!(a, b);

        #[section("second fails")] { assert!(false); }
    }

    #[section("first fails")] {
        let b = 1;
        assert_eq!(a, b);

        #[section("second succeeds")] {
            // The parent section fails assert so this never
            // gets executed.
            //
            // Should result in undefined result but doesn't.
            assert!(true);
        }
    }
}

// OUTPUT:
//
//   Running target/debug/deps/test-d4d1fd68ce8fee8b
//
//   running 5 tests
//   test example ... ok
//   test example_first_fails ... FAILED
//   test example_first_fails_second_succeeds ... FAILED
//   test example_first_succeeds ... ok
//   test example_first_succeeds_second_fails ... FAILED
```
