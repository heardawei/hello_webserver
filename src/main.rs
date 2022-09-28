use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = "GET / HTTP/1.1\r\n";

    if buffer.starts_with(&get.as_bytes()) {
        let html = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        );
        stream.write(response.as_bytes()).unwrap();
    } else {
        let html = fs::read_to_string("404.html").unwrap();
        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        );
        stream.write(response.as_bytes()).unwrap();
    }
}
