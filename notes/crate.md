# Managing Rust Crates

Rust uses a package manager called **Cargo** to manage dependencies, known as *crates*. Here’s a quick guide to managing crates in your Rust projects.

## Adding a Crate

To add a new crate, use:

```sh
cargo add crate_name
```

Or manually add it to your `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
```

## Updating Crates

Update all dependencies to the latest versions:

```sh
cargo update
```

To update a specific crate:

```sh
cargo update -p crate_name
```

## Removing a Crate

Remove a dependency with:

```sh
cargo remove crate_name
```

Or delete it from `Cargo.toml` and run:

```sh
cargo build
```

## Listing Dependencies

See all dependencies and their versions:

```sh
cargo tree
```

## Publishing Your Own Crate

1. Add metadata to `Cargo.toml`.
2. Login with `cargo login`.
3. Publish with:

    ```sh
    cargo publish
    ```

 ## Dev-Dependencies vs. Normal Dependencies

- **Dependencies** (`[dependencies]`): Needed for your crate to run. Included when your crate is used as a library.
- **Dev-dependencies** (`[dev-dependencies]`): Only required for development and testing (e.g., test frameworks). Not included when your crate is used as a dependency.

    Example in `Cargo.toml`:
    ```toml
    [dependencies]
    serde = "1.0"

    [dev-dependencies]
    rand = "0.8"
    ```

## Optional Features

You can enable or disable parts of a crate using features.

Example in `Cargo.toml`:
    ```toml
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    ```

To define your own features:
    ```toml
    [features]
    default = []
    extras = ["serde/derive"]
    ```

Enable features when building:
    ```sh
    cargo build --features extras
    ```

## Using Local or Git Dependencies

- **Local path dependency**:
        ```toml
        [dependencies]
        my_crate = { path = "../my_crate" }
        ```

- **Git repository dependency**:
        ```toml
        [dependencies]
        my_crate = { git = "https://github.com/username/my_crate.git", branch = "main" }
        ```
