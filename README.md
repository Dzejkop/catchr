# Catchr

A testing framework for Rust inspired by [Catch for C++](link).

Forget about `setup` and `teardown`. Forget about testing frameworks for `Rust` which attempt to mimic frameworks from other languages!. `Catchr` uses just one simple trick which will enable you to write concise, self-explaining and idiomatic tests for your project!

## Quickstart
Add `catchr = "0.1" to your `Cargo.toml` and start writing tests!

### Example using `catchr::scenarios!` proc macro

Write BDD-style tests using the `when`, `then`, etc. keywords.
```rust
#[cfg(test)]
mod tests {
    use my_crate::MyStruct;

    catchr::scenarios! {
        case "MyStruct tests" {
            // common setup code
            let mut my_struct = MyStruct::new();

            given "foo is called on my_struct" {
                my_struct.foo();

                then "my_struct.bar() should equal 1" {
                    // use regular Rust assertions
                    assert_eq!(my_struct.bar(), 1);
                }
            }

            given "foo is not called on my_struct" {
                then "my_struct.bar() should equal 0" {
                    assert_eq!(my_struct.bar(), 1);
                }
            }

            // common teardown code
            // and assertions veryfing invariants correct for all test paths

            assert!(my_struct.is_valid());
            my_struct.cleanup();
        }
    }
}
```

Every "top-level" section like `then "my_struct.bar() should equal 1" { ... }` will be resolved as a separate test case.
Each such test case will be located in a module reflecting the `case/when/then` structure.
Furthermore the code will follow scoping.

The above code will produce

```rust
#[cfg(test)]
#[allow(unused)] // added by catchr, for reasons explained later
mod tests {
    use my_crate::MyStruct;

    mod catchr_scenarios {
        use super::*;

        mod mystruct_tests {
            use super::*;

            mod when_foo_is_called_on_my_struct {
                use super::*;

                #[test]
                fn then_my_struct_bar_should_equal_1() {
                    let mut my_struct = MyStruct::new();
                    {
                        my_struct.foo();
                        {
                            assert_eq!(my_struct.bar(), 1);
                        }
                    }
                    assert!(my_struct.is_valid());
                    my_struct.cleanup();
                }
            }

            mod when_foo_is_not_called_on_my_struct {
                use super::*;

                #[test]
                fn then_my_struct_bar_should_equal_0() {
                    let mut my_struct = MyStruct::new();
                    {
                        my_struct.foo();
                        {
                            assert_eq!(my_struct.bar(), 0);
                        }
                    }
                    assert!(my_struct.is_valid());
                    my_struct.cleanup();
                }
            }
        }
    }
}
```

## Reference
`Catchr` tests are made up of `sections`. Each section starts with a keyword, has a message and a body.

### Supported section identifiers

|  keyword  |   produces    |
|-----------|---------------|
| `case`    | `case_xxx`    |
| `section` | `section_xxx` |
| `when`    | `when_xxx`    |
| `then`    | `then_xxx`    |
| `given`   | `given_xxx`   |

## Planned features

### Case enumeration
Enumerating through different input values with a `each $var_name in $values` section.

```rust
when "something" {
    each msg in ["A", "B", "C"] {
        assert_eq!(foo(msg), bar(msg));
    }
}
```

Will produce a test case for each of `msg` values
```rust
mod when_something {
    use super::*;

    #[test]
    fn msg_is_a() {
        let msg = "A";
        assert_eq!(foo(msg), bar(msg));
    }

    #[test]
    fn msg_is_b() {
        let msg = "B";
        assert_eq!(foo(msg), bar(msg));
    }

    #[test]
    fn msg_is_c() {
        let msg = "C";
        assert_eq!(foo(msg), bar(msg));
    }
}
```

### Opt-in better test output
If enabled with feature `catchr-messages`, each `catchr` generated test case will be prepended with a `CATCHR_MSG` constant. This constant can then be used in any assertions, printlns or dbgs, like so

```
section "Blah blah blah" {
    given "Something is true" {
        then "Something else is true" {
            assert!(false, CATCHR_MSG);
        }
    }
}
```

will produce

```rust
mod section_blah_blah_blah {
    use super::*;

    mod given_something_is_true {
        use super::*;

        const CATCHR_MSG: &str = "\nSection: Blah blah blah\n\tGiven: Something is true\n\t\tThen: Something else is true\n";
        #[test]
        fn then_something_else_is_true() {
            assert!(false, CATCHR_MSG);
        }
    }
}
```

which will fail with a nice message
```
---- section_blah_blah_blah::given_something_is_true::then_something_else_is_true stdout ----
thread 'section_blah_blah_blah::given_something_is_true::then_something_else_is_true' panicked at '
Section: Blah blah blah
	Given: Something is true
		Then: Something else is true
', src/lib.rs:11:13
```