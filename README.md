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
extern crate catch;
use catch::*;

#[catch_test]
fn example() {

    let a = 0;
    assert_eq!(a, 0);

    #[section("first assert succeeds")] {

        let b = 0;
        assert_eq!(a, b);

        #[section("second assert fails")] { assert!(false); }
    }

    #[section("first assert fails")] {

        let b = 1;
        assert_eq!(a, b);

        #[section("second assert succeeds")] {

            // The parent section fails assert so this never gets executed.
            // Should result in undefined result.
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
//   test example_first_assert_fails ... FAILED
//   test example_first_assert_fails_second_assert_succeeds ... FAILED
//   test example_first_assert_succeeds ... ok
//   test example_first_assert_succeeds_second_assert_fails ... FAILED
// 
```
