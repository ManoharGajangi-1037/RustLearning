// use std::{
//     fs,
//     io::{BufReader, prelude::*},
//     net::{TcpListener, TcpStream},
// };

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream);
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//     // println!("Request:{}",String::from_utf8_lossy(&buffer[..]))
//     // let contents=fs::read_to_string("index.html").unwrap();
//     // let content =match contents{
//     //     Ok(cont)=>cont,
//     //     Err(_)=>String::from("Error occured"),
//     // };
//     // let response ="HTTP/1.1 200 OK\r\n\r\n";
//     // let response=format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",contents.len(),contents);
//     let get = b"GET / HTTP/1.1\r\n";
//     if (buffer.starts_with(get)) {
//         let contents = fs::read_to_string("index.html").unwrap();
//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
//             contents.len(),
//             contents
//         );
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }else{
//         let contents = fs::read_to_string("404.html").unwrap();
//         let status_line="HTTP/1.1 404 NOT FOUND";
//         let response = format!(
//             "{}\r\nContent-Length:{}\r\n\r\n{}",
//             status_line,
//             contents.len(),
//             contents
//         );
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
// }
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let get2= b"GET /home HTTP/1.1\r\n";
    let (status_line, file_name) = if (buffer.starts_with(get)) {
        ("HTTP/1.1 200 0K", "index.html")
    } else if buffer.starts_with(get2){
        ("HTTP/1.1 200 0K", "home.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
