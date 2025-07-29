# Basic Syntax

## Common Functions

### Printing

- `println!()` — Prints to the console with a newline.
- `print!()` — Prints to the console without a newline.

#### String Interpolation

- Insert variables into strings using curly braces:
    ```rust
    println!("foo bar {}", y);
    ```
    - `y` is injected into the placeholder.
- For debugging (pretty-printing arrays, structs, etc.), use `{:?}`:
    ```rust
    println!("{:?}", arr);
    ```
- To control decimal places for floating-point numbers, use `:.n`:
    ```rust
    println!("{:.2}", pi); // prints pi to 2 decimal places
    ```

## Variables

### Declarations

- Basic syntax:
    ```rust
    let name: Type = value;
    ```
    - Example:
        ```rust
        let x: i32 = 42;
        ```
- Type annotation is optional if the compiler can infer the type:
    ```rust
    let y = 3.14;
    ```
- Variables are immutable by default. Use `mut` for mutability:
    ```rust
    let mut counter = 0;
    ```