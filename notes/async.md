# Async/Await in Rust

## Async Functions: Basics

- Add the `async` keyword to a function as usual:
    ```rust
    async fn my_async_func() { /* ... */ }
    ```
- Async functions return a `Future` (similar to a Promise in other languages).
- To get the result, use `.await`:
    ```rust
    let value = async_func().await;
    ```
- Async functions must be run inside an async runtime, e.g.:
    ```rust
    #[tokio::main]
    async fn main() {
        // ...
    }
    ```

## Sequential vs Parallel Execution

### Sequential Execution

Each async call waits for the previous one to finish:

```rust
let a = fetch_a().await;
let b = fetch_b().await;
```

### Parallel Execution

Start both futures, then await them together:

```rust
let a = fetch_a();
let b = fetch_b();
let (a_res, b_res) = tokio::join!(a, b); // both run at the same time
```