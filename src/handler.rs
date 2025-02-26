use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    println!("Request: {}", request_line);

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nHello, Rust server!";
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // <- ここを追加
}