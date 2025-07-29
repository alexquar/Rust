# Rust Data Types (Dtypes)

## Basics

- **Mutable Variables:**  
    Use `mut` to make a variable mutable:  
    ```rust
    let mut name = value;
    ```
- **Variable Declaration:**  
    Use `let` to declare variables (immutable by default, but can be shadowed):  
    ```rust
    let x = 5;
    let x = x + 1; // shadowing
    ```
- **Constants:**  
    Use `const` for compile-time constants (must specify type):  
    ```rust
    const THING: i32 = 10_000;
    ```
- **Static Variables:**  
    Stored in global memory, fixed address, can be mutable with `unsafe`:  
    ```rust
    static mut THING: i32 = 0;
    unsafe { THING += 1; }
    ```

## Primitive Types

### Integers

- Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

### Floats

- `f32`, `f64`

### Booleans

- `true` / `false`

### Characters

- `char` (single quotes, Unicode scalar values)

### None

- Similar to Python's `None`, use `Option<T>` in Rust:  
    ```rust
    let x: Option<i32> = None;
    if x == None { /* ... */ }
    ```

## Compound Types

### Arrays

- Fixed-size, same type elements:  
    ```rust
    let numbers: [i32; 3] = [1, 2, 3];
    let first = numbers[0];
    ```

### Tuples

- Can contain mixed types:  
    ```rust
    let thing: (&str, i32, bool) = ("Thing", 20, true);
    let name = thing.0;
    ```
- Type annotation is optional.

### Vectors

- Growable, heap-allocated arrays:  
    ```rust
    let nums = vec![1, 2, 3];
    ```
- Common methods:  
    - `push`, `pop`, `len`, `get`, etc.

## String Types

### String Slices (`&str`)

- Immutable reference to a string, not owned, static or borrowed:  
    ```rust
    let s: &str = "hello";
    ```

### `String`

- Growable, heap-allocated, owned string:  
    ```rust
    let mut thing: String = String::from("hello");
    thing.push_str(" world");
    let len = thing.len();
    let subset = &thing[0..5];
    ```

