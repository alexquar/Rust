# Testing

Rust provides a built-in test framework to help you write and run tests. To create tests, add a `tests` module annotated with `#[cfg(test)]` in your source files:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

To run tests, use:

```sh
cargo test
```