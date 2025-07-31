use std::{fs, io::{
    prelude::*, BufReader
}, net::{
    TcpListener, TcpStream
}};
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
                hande_connection(stream);
            }
            Err(e) => {
                print_error(&format!("Failed to establish a connection: {}", e));
            }
        }
    }
}

fn hande_connection(mut stream: TcpStream){
   let buf_reader = BufReader::new(&stream);
   let request_line = buf_reader.lines().next().unwrap().unwrap();
   if request_line == "GET / HTTP/1.1" {
    let status = "HTTP/1.1 200 OK\r\n";
    let contents = fs::read_to_string("./pages/home.html").expect("Failed to read home.html. Check the working directory above.");
    let length = contents.len();
    let response = format!("{status}Content-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");  
    let result = stream.write_all(response.as_bytes());
    match result {
            Ok(_) => println!("Response sent successfully."),
            Err(e) => print_error(&format!("Failed to send response: {}", e)),
        }
    } else if request_line == "GET /settings HTTP/1.1" {
        let status = "HTTP/1.1 200 OK\r\n";
        let contents = fs::read_to_string("./pages/settings.html").expect("Failed to read settings.html. Check the working directory above.");
        let length = contents.len();
        let response = format!("{status}Content-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");
        let result = stream.write_all(response.as_bytes());
        match result {
            Ok(_) => println!("Response sent successfully."),
            Err(e) => print_error(&format!("Failed to send response: {}", e)),
        }
    } else {
        let status = "HTTP/1.1 404 Not Found\r\n";
        let contents = fs::read_to_string("./pages/404.html").expect("Failed to read 404.html. Check the working directory above.");
        let length = contents.len();
        let response = format!("{status}Content-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");
        let result = stream.write_all(response.as_bytes());
        match result {
            Ok(_) => println!("Response sent successfully."),
            Err(e) => print_error(&format!("Failed to send response: {}", e)),
        }
    }
}