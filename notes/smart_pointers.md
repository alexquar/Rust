# Rust Smart Pointers

Rust provides several smart pointer types in its standard library, each with unique features and use-cases. This guide covers:

- Reference Counting (`Rc`)
- Atomic Reference Counting (`Arc`)
- Interior Mutability (`RefCell`, `Cell`)
- Mutexes (`Mutex`)
- Copy-only Interior Mutability (`Cell`)
- Other relevant smart pointers

---

## 1. Reference Counting (`Rc<T>`)

`Rc<T>` enables multiple ownership of data by keeping a reference count. When the count drops to zero, the value is dropped.

- **Use case:** Shared read-only data in single-threaded scenarios.

```rust
use std::rc::Rc;

/// Demonstrates multiple ownership using Rc<T>.
fn main() {
    // Create an Rc pointer to an integer.
    let a = Rc::new(5);
    // Clone increases the reference count.
    let b = Rc::clone(&a);
    println!("a: {}, b: {}", a, b);
    // Rc::strong_count returns the number of strong references.
    println!("Reference count: {}", Rc::strong_count(&a));
}
```
**Explanation:**
- `Rc::new(value)` creates a new reference-counted pointer.
- `Rc::clone(&rc)` creates another pointer to the same data, incrementing the count.
- `Rc::strong_count(&rc)` returns the current reference count.
- When all `Rc` pointers go out of scope, the value is dropped.

**Key Points:**
- Not thread-safe.
- Cloning increases the count; dropping decreases it.

---

## 2. Atomic Reference Counting (`Arc<T>`)

`Arc<T>` is like `Rc<T>`, but thread-safe using atomic operations.

- **Use case:** Shared read-only data across threads.

```rust
use std::sync::Arc;
use std::thread;

/// Demonstrates sharing data across threads using Arc<T>.
fn main() {
    // Create an Arc pointer to a vector.
    let data = Arc::new(vec![1, 2, 3]);
    // Spawn multiple threads, each with a clone of the Arc.
    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("Thread {}: {:?}", i, data);
        })
    }).collect();

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}
```
**Explanation:**
- `Arc::new(value)` creates a new atomic reference-counted pointer.
- `Arc::clone(&arc)` clones the pointer for sharing across threads.
- Each thread receives its own `Arc`, ensuring thread safety.

**Key Points:**
- Thread-safe.
- Slightly slower than `Rc` due to atomic operations.

---

## 3. Interior Mutability

Interior mutability allows mutation through an immutable reference. Rust provides `Cell<T>` and `RefCell<T>` for this.

### `RefCell<T>`

- **Use case:** Mutability checked at runtime, not compile time. Only for single-threaded code.

```rust
use std::cell::RefCell;

/// Demonstrates interior mutability with RefCell<T>.
fn main() {
    // Create a RefCell containing an integer.
    let data = RefCell::new(5);
    // Mutably borrow the value and increment it.
    *data.borrow_mut() += 1;
    // Immutably borrow the value for printing.
    println!("data: {}", data.borrow());
}
```
**Explanation:**
- `RefCell::new(value)` creates a new RefCell.
- `borrow_mut()` returns a mutable reference, checked at runtime.
- `borrow()` returns an immutable reference.
- Violating borrowing rules (e.g., two mutable borrows) causes a panic at runtime.

**Key Points:**
- Panics at runtime if borrowing rules are violated.
- Not thread-safe.

### `Cell<T>`

- **Use case:** Copy types only. No references to inner data.

```rust
use std::cell::Cell;

/// Demonstrates interior mutability with Cell<T>.
fn main() {
    // Create a Cell containing an integer.
    let data = Cell::new(5);
    // Set a new value.
    data.set(10);
    // Get the current value.
    println!("data: {}", data.get());
}
```
**Explanation:**
- `Cell::new(value)` creates a new Cell.
- `set(value)` replaces the value.
- `get()` returns a copy of the value.
- Only works for types that implement `Copy`.

**Key Points:**
- For `Copy` types.
- No borrowing, just get/set.

---

## 4. Mutexes (`Mutex<T>`)

`Mutex<T>` provides interior mutability with thread safety.

- **Use case:** Shared mutable state across threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

/// Demonstrates thread-safe shared mutability with Mutex<T>.
fn main() {
    // Create an Arc pointer to a Mutex-protected integer.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn 10 threads, each incrementing the counter.
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // Lock the mutex to get mutable access.
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final counter value.
    println!("Result: {}", *counter.lock().unwrap());
}
```
**Explanation:**
- `Mutex::new(value)` creates a mutex-protected value.
- `lock()` acquires the lock, returning a guard for mutable access.
- `Arc` is used to share the mutex across threads.
- Always handle potential panics from `lock()` (e.g., with `unwrap()`).

**Key Points:**
- Locks data for exclusive access.
- Can deadlock if not used carefully.

---

## 5. Copy-only Interior Mutability (`Cell<T>`)

As above, `Cell<T>` is for types that implement `Copy`. It allows mutation without borrowing.

---

## 6. Other Smart Pointers

### `Box<T>`

- Heap allocation for single ownership.

```rust
// Box<T> allocates data on the heap and provides single ownership.
let b = Box::new(5);
```
**Explanation:**
- `Box::new(value)` allocates `value` on the heap.
- The box is dropped when it goes out of scope, deallocating the heap memory.

### `Weak<T>`

- Non-owning reference to data managed by `Rc` or `Arc`.

```rust
use std::rc::{Rc, Weak};

/// Demonstrates creating a weak reference from an Rc<T>.
let strong = Rc::new(5);
// Rc::downgrade creates a Weak pointer that does not affect the reference count.
let weak: Weak<i32> = Rc::downgrade(&strong);
```
**Explanation:**
- `Rc::downgrade(&rc)` creates a `Weak` pointer.
- `Weak` pointers do not keep the value alive.
- Upgrading a `Weak` pointer returns `Some(Rc<T>)` if the value is still alive, or `None` if it has been dropped.

---

## Summary Table

| Type      | Thread-safe | Multiple Owners | Interior Mutability | Use-case                |
|-----------|-------------|----------------|---------------------|-------------------------|
| `Box<T>`  | N/A         | No             | No                  | Heap allocation         |
| `Rc<T>`   | No          | Yes            | No                  | Shared ownership        |
| `Arc<T>`  | Yes         | Yes            | No                  | Shared across threads   |
| `RefCell` | No          | No             | Yes                 | Runtime-checked mut.    |
| `Cell`    | No          | No             | Yes (Copy only)     | Copy-only mutability    |
| `Mutex`   | Yes         | Yes            | Yes                 | Thread-safe mutability  |
| `Weak`    | Yes/No      | No             | No                  | Non-owning references   |

---

## References

- [Rust Book: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust Docs: std::rc, std::sync, std::cell](https://doc.rust-lang.org/std/)

