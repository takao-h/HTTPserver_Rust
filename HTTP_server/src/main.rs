use std::fmt::format;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::stream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}

fn handle_connection(mut _stream: TcpStream) {
    let mut buffer = [0; 1024];
    _stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let put: &[u8; 16] = b"PUT / HTTP/1.1\r\n";
    let post: &[u8; 17] = b"POST / HTTP/1.1\r\n";
    let delete: &[u8; 19] = b"DELETE / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        _stream.write(response.as_bytes()).unwrap();
        _stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("404.html").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        _stream.write(response.as_bytes()).unwrap();
        _stream.flush().unwrap();
    }
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    _stream.write(response.as_bytes()).unwrap();
    _stream.flush().unwrap();

}
