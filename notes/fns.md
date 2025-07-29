# Functions

## Basics

### Creation

```rust
fn name() {}
```

#### With Arguments

```rust
fn name(arg: Type) {}
```
- Use string slices (`&str`) for passing strings.

### Calling

```rust
name();
name(arg1, arg2);
```

### Returns

- Use `return` to return a value, or omit the semicolon on the last expression to return its value implicitly.

```rust
fn calc() -> i32 {
    5 * 10 // returns 50
}
```

- Example with a block expression:

```rust
let x = {
    5 * 10
}; // x is 50
```

- Explicit return type:

```rust
fn thing() -> i32 {
    // ...
}
```