# Custom Types in Rust

## Structs

### Declaration

```rust
struct Name {
    field1: Type,
    field2: Type,
}
```

### Creation

```rust
let thing = Name {
    field1: value1,
    field2: value2,
};
```

### Implementations

- Allows you to define functions (methods) on structs.
- Use `impl` to implement methods:

```rust
impl Name {
    fn method1(&self) {
        // method body
    }

    fn method2(&mut self, value: Type) {
        self.field1 = value;
    }

    // Constructor pattern
    fn new(field1: Type, field2: Type) -> Self {
        Self { field1, field2 }
    }

    // Example of a cleanup method
    fn deactivate(&mut self) {
        // cleanup code
    }
}
```

- `self` gives access to member variables.
- Methods are called like:

```rust
let mut thing = Name::new(val1, val2);
thing.deactivate();
```

---

## Enums

### Declaration

```rust
enum Thing {
    Variant1,
    Variant2,
}
```

### Creation

```rust
let thing = Thing::Variant1;
```

---

## Traits (Interfaces)

- Define a trait:

```rust
trait Something {
    fn something(&self);
}
```

- Implement the trait for a struct:

```rust
impl Something for Name {
    fn something(&self) {
        // implementation
    }
}
```

- Use trait methods with dot notation: `thing.something()`

---

## Methods vs Functions in Rust

- **Methods**: Associated functions that operate on a specific instance (take `&self` or `&mut self`).
- **Functions**: Standalone or associated with a type, but do not take `self`.

**Examples:**

```rust
// Function (associated, no self)
let s = String::from("hello");

// Method (called on an instance)
let len = s.len();
```