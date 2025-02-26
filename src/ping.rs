use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use crate::junction::router;
use crate::pong::send_response;

pub fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    
    let read_line = reader.read_line(&mut request_line);

    if let Err(_) = read_line {
        send_response(400, stream, "");
        return;
    }

    let method = request_line.split_whitespace().next();
    let uri = request_line.split_whitespace().nth(1);

    if let (Some(method), Some(uri)) = (method, uri) {
        if method != "GET" {
            send_response(405, stream, "");
            println!("Unsupported method: {}", method);
            return;
        }
    
        println!("Request: {}", request_line);
    
        router(uri, stream);
    } else {
        send_response(400, stream, "");
        return;
    }

}