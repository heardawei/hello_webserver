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

    let (html, status_line) = if buffer.starts_with(&get.as_bytes()) {
        ("index.html", "HTTP/1.1 200 OK\r\n")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND\r\n")
    };
    let html = fs::read_to_string(html).unwrap();
    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        html.len(),
        html
    );
    stream.write(response.as_bytes()).unwrap();
}
