# Modules and Structure

## Project Structure

### Main File (`main.rs`)
- Entry point of the application.
- To include code from another file:
    ```rust
    mod file_name;
    ```
- To use public functions from another module:
    ```rust
    use file_name::func_name;
    // or import multiple functions
    use file_name::{
            func1,
            func2,
    };
    ```

### Modules
- Modules allow organizing code across multiple files.
- To make a function accessible from outside its module, declare it as public:
    ```rust
    pub fn thing() {}
    ```

### Imports
- Import a single function:
    ```rust
    use mod_name::func;
    ```
- Glob import (import everything from a module or type):
    ```rust
    use mod_name::type_name::*;
    ```
- Rename imports:
    ```rust
    use std::collections::func as col_func;
    ```

### Dependencies
- `Cargo.toml` lists dependencies and their versions.
- To add a dependency, specify it in `Cargo.toml`:
    ```toml
    [dependencies]
    crate_name = "version"
    ```
- No manual install step; dependencies are downloaded automatically at compile time when you build with Cargo.
- Use `use` to import items from dependencies in your code.