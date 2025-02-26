use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use crate::pong::send_response;

pub fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    let method = request_line.split_whitespace().next().unwrap();
    // let uri = request_line.split_whitespace().nth(1).unwrap();

    if method != "GET" {
        let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
        send_response(stream, response);
        println!("Unsupported method: {}", method);
        return;
    }

    println!("Request: {}", request_line);

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nHello, Rust server!";
    send_response(stream, response);
}