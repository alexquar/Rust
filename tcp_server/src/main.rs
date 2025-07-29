use std::{io::{
    BufReader, 
    prelude::*
}, net::{
    TcpStream, 
    TcpListener
}};
mod utils;
use utils::print_error;

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
   let response = "HTTP/1.1 200 OK\r\n\r\n";
   stream.write_all(response.as_bytes()).unwrap();
}