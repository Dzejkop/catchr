# Catchr ![Rust](https://github.com/Dzejkop/catchr/workflows/Rust/badge.svg)

_Experimental: Might eat your laundry!_

A testing framework for Rust inspired by [Catch for C++](https://github.com/catchorg/Catch2).

## Quickstart

### Add `catchr = "0.2.1"` to your `Cargo.toml`

### Write tests:

```rust
#[cfg(test)]
mod tests {
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
}
```

### `cargo test`

```
running 2 tests
test tests::section_my_tests::given_x_is_equal_to_1::when_2_is_added_to_x::then_x_should_equal_3 ... ok
test tests::section_my_tests::given_x_is_equal_to_1::when_1_is_added_to_x::then_x_should_equal_2 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Sections

Each test section consists of a keyword, a description and a body.

```rust
keyword "description" {
    // body
}
```

For the moment the following keywords are supported: `section`, `case`, `when`, `then`, `given`.

Sections without any nested section will become test cases. Sections function like scopes - that is statements from the outer section are available in the inner section:

```rust
when "something" {
    let x = 1;

    then "anything" {
        let y = 1;
        assert_eq!(x, y);
    }

    then "whatever" {
        assert!(true);
    }
}
```

The `let x = 1;` can be used in the `then "anything"` section. But `let y = 1;` from the `then "anything"` section, cannot be used in the `then "whatever"` section.

Furthermore the scoping rules are preserved, so that inner test cases can borrow mutably without violating the borrow checker rules.

Consider the following example:

```rust
case "a" {
    let mut tmp = tempfile().unwrap();

    case "should write some data" {
        let mut writer = BufWriter::new(&mut tmp);
        writer.write_all(&[1, 2, 3]).unwrap();
    }

    tmp.seek(SeekFrom::Start(0)).unwrap();
    let bytes_in_tmp_file = tmp.seek(SeekFrom::End(0)).unwrap();

    assert_eq!(bytes_in_tmp_file, 3);
}
```

if the test case was expanded without scoping, we'd get

```rust
let mut tmp = tempfile().unwrap();

let mut writer = BufWriter::new(&mut tmp);
writer.write_all(&[1, 2, 3]).unwrap();

tmp.seek(SeekFrom::Start(0)).unwrap();
let bytes_in_tmp_file = tmp.seek(SeekFrom::End(0)).unwrap();

assert_eq!(bytes_in_tmp_file, 3);
```

which fails to compile!

so `catchr` will expand this test case into

```rust
let mut tmp = tempfile().unwrap();

{
    let mut writer = BufWriter::new(&mut tmp);
    writer.write_all(&[1, 2, 3]).unwrap();
}

tmp.seek(SeekFrom::Start(0)).unwrap();
let bytes_in_tmp_file = tmp.seek(SeekFrom::End(0)).unwrap();

assert_eq!(bytes_in_tmp_file, 3);
```

## How does it work?

The code from the [**Quickstart**](##Quickstart) section will expand into something like this:

```rust
#[cfg(test)]
mod tests {
    mod section_my_tests {
        use super::*;

        mod given_x_is_equal_to_1 {
            use super::*;

            mod when_1_is_added_to_x {
                use super::*;

                #[test]
                fn then_x_should_equal_2() {
                    {
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
            }

            mod when_2_is_added_to_x {
                use super::*;

                #[test]
                fn then_x_should_equal_3() {
                    {
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
    }
}
```
