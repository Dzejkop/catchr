# Catchr ![Rust](https://github.com/Dzejkop/catchr/workflows/Rust/badge.svg)
*Experimental: Might eat your laundry!*

A testing framework for Rust inspired by [Catch for C++](https://github.com/catchorg/Catch2).

## Quickstart

### Add `catchr = "0.2.0"` to your `Cargo.toml`

### Write tests:
```rust
catchr::describe! {
    section "my tests" {
        given "x is equal to 1" {
            let mut x = 1;

            when "1 is added to x" {
                x += 1;

                then "x should equal 2" {
                    assert_eq!(2, x);
                }
            }

            when "2 is added to x" {
                x += 2;

                then "x should equal 3" {
                    assert_eq!(3, x);
                }
            }

            // for all code paths
            assert!(x >= 2);
        }
    }
}
```

### `cargo test`
```
running 2 tests
test tests::section_my_tests::given_x_is_equal_to_1::when_2_is_added_to_x::then_x_should_equal_3 ... ok
test tests::section_my_tests::given_x_is_equal_to_1::when_1_is_added_to_x::then_x_should_equal_2 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## How does it work?

The code above will produce something like this
```rust
mod section_my_tests {
    use super::*;

    mod given_x_is_equal_to_1 {
        use super::*;

        mod when_1_is_added_to_x {
            use super::*;

            #[test]
            fn then_x_should_equal_2() {
                let mut x = 1;
                {
                    x += 1;
                    {
                        assert_eq!(2, x);
                    }
                }
                assert!(x >= 2);
            }
        }

        mod when_2_is_added_to_x {
            use super::*;

            #[test]
            fn then_x_should_equal_3() {
                let mut x = 1;
                {
                    x += 2;
                    {
                        assert_eq!(3, x);
                    }
                }
                assert!(x >= 2);
            }
        }
    }
}
```
