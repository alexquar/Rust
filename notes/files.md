# Working with Files in Rust

## Opening a File

```rust
use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Error> {
    let file = File::open("foo.txt")?;
    Ok(())
    //Ok is an enum variant used in Rust's Result type, which represents either success (Ok) or failure (Err).
// () is the unit type in Rust, meaning "no value" or "nothing".
// So, Ok(()) means "an operation succeeded, and there is no meaningful value to return."
// It's commonly used when a function returns Result<(), ErrorType> to indicate success without data.
}
```

## Reading from a File

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
```

## Writing to a File

```rust
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("bar.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
```

## Appending to a File

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open("bar.txt")?;
    file.write_all(b"\nAppended text")?;
    Ok(())
}
```