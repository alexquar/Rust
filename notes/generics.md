# Rust Generics Notes

## What are Generics?

Generics allow you to write flexible, reusable code for multiple data types.

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

## Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

let int_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };
```

## Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## Traits and Trait Bounds

Use trait bounds to specify what capabilities a generic type must have.

```rust
fn print<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}
```

Multiple trait bounds:

```rust
fn compare<T: PartialEq + PartialOrd>(a: T, b: T) {
    // ...
}
```

## Where Clauses

For complex bounds, use `where`:

```rust
fn foo<T, U>(t: T, u: U)
where
    T: Clone,
    U: std::fmt::Debug,
{
    // ...
}
```

## Monomorphization

Rust generates concrete types at compile time for each generic use, so there's no runtime cost.

---

**References:**
- [Rust Book: Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust By Example: Generics](https://doc.rust-lang.org/rust-by-example/generics.html)