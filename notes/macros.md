## Rust Macros

Rust macros are powerful metaprogramming tools that allow you to write code that writes other code. They come in two main forms:

- **Declarative macros** (macro_rules!): Pattern-based macros that match against code syntax and expand into Rust code.
- **Procedural macros**: Functions that operate on Rust code as input and produce Rust code as output. These include custom derive, attribute-like, and function-like macros.

### Example: Declarative Macro

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

say_hello!(); // Expands to println!("Hello!");
```

### When to Use Macros

- To reduce code duplication.
- To implement domain-specific languages.
- For advanced compile-time code generation.

For most cases, prefer functions and generics. Use macros when you need capabilities that functions can't provide.