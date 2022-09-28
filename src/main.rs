use hello_webserver::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (html, status_line) = if buffer.starts_with(get) {
        ("index.html", "HTTP/1.1 200 OK\r\n")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
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
