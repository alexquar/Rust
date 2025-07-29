# Borrowing and Ownership

## Ownership

### Ownership Rules

- Each value in Rust has a single owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value is dropped.
- You can have either one mutable reference or multiple immutable references to a value at a time.

---

## References

- References are immutable by default.
- They allow you to borrow a value without taking ownership (they are just pointers).
- Syntax: `&variable_name`
- Function parameter: `&Type`

---

## Mutable References

- To mutate through a reference, both the original value and the reference must be mutable.
    ```rust
    let mut x: i32 = 5;
    let r: &mut i32 = &mut x;
    *r += 1; // Dereference to modify the value
    ```
- You can only have one mutable reference to a value in a particular scope.

---

## Ownership Transfer

- Assigning ownership:
    ```rust
    let s1 = String::from("RUST");
    let s2 = s1; // s1 is no longer valid, s2 owns the value
    ```
- Passing by value to a function transfers ownership.
- Passing by reference (`&value`) allows borrowing, so the original owner retains ownership.

---

## Borrowing

- Borrowing involves temporary access to a value through references (`&obj`).
- To borrow mutably, use `&mut`:
    ```rust
    fn modify(val: &mut i32) {
        *val += 1;
    }
    ```
- Only one mutable reference or multiple immutable references are allowed at a time.


