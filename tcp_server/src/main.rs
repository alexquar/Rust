use std::{fs, io::{
    prelude::*, BufReader
}, net::{
    TcpListener, TcpStream
},
thread,
time::Duration,
collections::HashMap
};
mod utils;
use utils::{
    print_error,
};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                handle_connection(stream);
            }
            Err(e) => {
                print_error(&format!("Failed to establish a connection: {}", e));
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {

    // Define a map of routes to their corresponding HTML files
    let mut routes: HashMap<&str, &str> = HashMap::new();
    routes.insert("/", "./pages/home.html");
    routes.insert("/settings", "./pages/settings.html");

    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => {
            print_error("Failed to read request line.");
            return;
        }
    };

    // Parse the path from the request line
    let path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/");

    let (status, file_path) = if let Some(&file) = routes.get(path) {
        ("HTTP/1.1 200 OK\r\n", file)
    } else {
        ("HTTP/1.1 404 Not Found\r\n", "./pages/404.html")
    };

    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => {
            print_error(&format!("Failed to read {}. Check the working directory above.", file_path));
            return;
        }
    };
    let length = contents.len();
    thread::sleep(Duration::from_secs(5));
    let response = format!("{status}Content-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");
    let result = stream.write_all(response.as_bytes());
    match result {
        Ok(_) => println!("Response sent successfully."),
        Err(e) => print_error(&format!("Failed to send response: {}", e)),
    }
}