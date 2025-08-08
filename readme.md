# Custom Rust Webserver

This project is a custom-built webserver written in Rust. For implementation details, please refer to the code base.

## About 

- Multi threaded webserver custom built using a threadpool 
- Serves HTML to set route
- Allows a statically set number of threads 
- Features flawless shutdown through custom drop methods of workers 

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (comes with Rust)

### Building and Running

Clone the repository and navigate to the project directory:

```sh
git clone <repo-url>
cd <project-directory>
```

Build and run the server:

```sh
cargo run
```

### Manual Compilation

If you want to compile and run manually:

```sh
rustc src/main.rs
./main
```

## Project Structure

- `src/main.rs`: Entry point of the webserver.
- Additional modules and configuration files as needed.

## Usage

Once running, the server will listen on the configured port. Check the code base for endpoints and further customization.
