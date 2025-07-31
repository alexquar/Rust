# Concurrency

Concurrency in Rust enables multiple computations to make progress independently. Rust provides safety guarantees through its ownership and type system, preventing data races at compile time.

## Threads

Spawn a new thread using `std::thread`:

```rust
use std::thread; // Import the thread module from the standard library

fn main() {
    // Spawn a new thread. The closure (|| { ... }) is the code that will run in the new thread.
    let handle = thread::spawn(|| {
        println!("Hello from a thread!"); // This line runs in the spawned thread.
    });

    // Wait for the spawned thread to finish. 'join' returns a Result, so we call 'unwrap' to panic if the thread panicked.
    handle.join().unwrap();
}
```
**Explanation:**
- `thread::spawn` creates a new OS thread and runs the closure in it.
- The `handle` allows you to wait for the thread to finish with `join`.
- `unwrap()` will panic if the thread panicked.

---

## Message Passing

Use channels for safe communication between threads:

```rust
use std::sync::mpsc; // Import the multi-producer, single-consumer channel module
use std::thread;

fn main() {
    // Create a channel. 'tx' is the sending end, 'rx' is the receiving end.
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread, moving the sending end (tx) into it.
    thread::spawn(move || {
        tx.send("Hello").unwrap(); // Send a message through the channel. 'unwrap' panics if the receiver is dropped.
    });

    // Receive the message from the channel. 'recv' blocks until a message is available.
    println!("Received: {}", rx.recv().unwrap());
}
```
**Explanation:**
- `mpsc::channel()` creates a channel for sending data between threads.
- `tx.send("Hello")` sends a message from the spawned thread.
- `rx.recv()` waits for and receives the message in the main thread.

---

## Shared State

Use `Arc` and `Mutex` for shared mutable state:

```rust
use std::sync::{Arc, Mutex}; // Import Arc (atomic reference counting) and Mutex (mutual exclusion)
use std::thread;

fn main() {
    // Create a counter protected by a Mutex, wrapped in an Arc for shared ownership across threads.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![]; // Store thread handles to join later

    for _ in 0..10 {
        // Clone the Arc to get a new reference for each thread.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the mutex to get mutable access to the counter.
            let mut num = counter.lock().unwrap();
            *num += 1; // Increment the counter
        });
        handles.push(handle); // Save the handle for joining
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the mutex again to read the final value.
    println!("Result: {}", *counter.lock().unwrap());
}
```
**Explanation:**
- `Arc` lets multiple threads own the same data.
- `Mutex` ensures only one thread can access the data at a time.
- Each thread locks the mutex, increments the counter, and unlocks when done.
- The main thread waits for all threads to finish, then prints the result.

---

## async/await

Rust supports asynchronous programming with `async`/`await` (requires an async runtime like `tokio` or `async-std`):

```rust
use tokio::time::{sleep, Duration}; // Import sleep and Duration for async timing

#[tokio::main] // Macro that sets up the async runtime and runs the main function
async fn main() {
    // Spawn an asynchronous task. The closure is async, so it can use .await.
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await; // Wait asynchronously for 1 second
        println!("Hello from async task!");
    });

    // Wait for the async task to finish. 'await' yields control until the task completes.
    handle.await.unwrap();
}
```
**Explanation:**
- `tokio::spawn` runs an async task in the background.
- `sleep(Duration::from_secs(1)).await` pauses the task asynchronously.
- `handle.await` waits for the task to finish.

---

## Resources

- [The Rust Book: Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Async Programming](https://rust-lang.github.io/async-book/)