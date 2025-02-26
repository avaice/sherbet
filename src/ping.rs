use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use crate::junction::router;
use crate::pong::send_response;

pub fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    let method = request_line.split_whitespace().next().unwrap();
    let uri = request_line.split_whitespace().nth(1).unwrap();

    if method != "GET" {
        send_response(405, stream, "");
        println!("Unsupported method: {}", method);
        return;
    }

    println!("Request: {}", request_line);

    router(uri, stream);

}