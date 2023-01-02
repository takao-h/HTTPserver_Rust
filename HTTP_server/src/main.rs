use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        // println!("Connection established!");
        handle_connection(_stream);
    }
}

fn handle_connection(mut _stream: TcpStream) {
    let mut buffer = [0; 1024];
    _stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}