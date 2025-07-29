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
   let http_request: Vec <_> = buf_reader.lines().map(|line| line.unwrap()).take_while(|line| !line.is_empty()).collect();
   println!("HTTP Request: {:?}", http_request);
   let status = "HTTP/1.1 200 OK\r\n";
   use std::env;
   println!("Current working directory: {:?}", env::current_dir().unwrap());
   let contents = fs::read_to_string("home.html").expect("Failed to read home.html. Check the working directory above.");
   let length = contents.len();
   let response = format!("{status}Content-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");  
   let result = stream.write_all(response.as_bytes());
   match result {
         Ok(_) => println!("Response sent successfully."),
         Err(e) => print_error(&format!("Failed to send response: {}", e)),
    }
}