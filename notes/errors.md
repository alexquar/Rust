# Errors

## Handling

### Option

- Represents a value that can be either `Some(T)` or `None`.
- Example:
    ```rust
    fn get(index: usize) -> Option<i32> {
        let nums = vec![1, 2, 3];
        nums.get(index).cloned()
    }
    ```
- You need to check if the value is `Some(i32)` or `None`:
    ```rust
    match get(1) {
        Some(val) => println!("Value: {}", val),
        None => println!("No value found"),
    }
    ```
- The `Option` enum:
    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```

### Panic & Assert

- Terminates the current thread (aborts execution).
    ```rust
    panic!("Something went super wrong");
    ```

- Assert words like normal, it checks a value and then panics with the provided message if it fails 
    ```rust
    assert!(size > 0, "Thread pool size must be greater than zero.");
    ```

### Recoverable Errors

- Used for expected failures (e.g., file parsing, network issues).
- Rust uses the `Result<T, E>` type:
    ```rust
    enum Result<T, E> {
        Ok(T),   // success
        Err(E),  // failure
    }
    ```
- Use a `match` statement to handle success or error:
    ```rust
    use std::fs::File;

    let result = File::open("thing.txt");
    match result {
        Ok(file) => println!("{:?}", file),
        Err(e) => println!("Failure: {:?}", e),
    }
    ```
- You can use `.unwrap()` to get the value or panic on error:
    ```rust
    let file = File::open("file.txt").unwrap();
    ```
- Or use `.expect("Message")` to customize the panic message:
    ```rust
    let file = File::open("file.txt").expect("Failed to open file");
    ```
- The `?` operator propagates errors up the call stack:
    ```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn read_file() -> Result<String, io::Error> {
        let mut file = File::open("hello.txt")?;  // returns early on error
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;      // returns early on error
        Ok(contents)
    }
    ```
    - If any operation fails, the error is returned immediately.
