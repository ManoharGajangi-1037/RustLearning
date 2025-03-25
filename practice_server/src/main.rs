use std::fmt::format;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    println!("Hello, world!");
}
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Requestss,{:?}",buffer[..])
    let get = b"GET / HTTP/1.1";
    let (status_line, file_name) = if (buffer.starts_with(get)) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    // let status_line = "HTTP/1.1 /home 200 OK";
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContentLength: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
