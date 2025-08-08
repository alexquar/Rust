# Testing

Rust includes a powerful built-in test framework to help you ensure your code works as expected. You can write unit tests directly in your source files by adding a `tests` module annotated with `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_fails() {
        assert_ne!(2 * 2, 5);
    }
}
```

- Use `#[test]` to mark functions as tests.
- Use assertions like `assert!`, `assert_eq!`, and `assert_ne!` to check conditions.

To run all tests in your project, use:

```sh
cargo test
```

This command compiles your code in test mode and runs all functions marked with `#[test]`. Test output will show which tests passed or failed.

**Tips:**
- Place unit tests in the same file as the code they test, inside a `#[cfg(test)]` module.
- For integration tests, create separate files in the `tests/` directory at the project root.
- You can run a specific test by passing its name to `cargo test`:

  ```sh
  cargo test it_works
  ```

For more details, see the [Rust Book: Testing](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).