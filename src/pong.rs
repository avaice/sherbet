use std::io::Write;
use std::net::TcpStream;

use mime_guess::Mime;

use crate::factory::header_factory;

pub fn send_response(uri: &str, status: u16, mut stream: TcpStream, response: &str) {
    let val = format!("HTTP/1.1 {}\r\n{}", status, response);
    let _ = stream.write_all(val.as_bytes());
    let _ = stream.flush();
    println!("{} {}", status, uri);
}

pub fn send_binary_response(status: u16, mut stream: TcpStream, response: &[u8], mime_type: &Mime) {
    let val = format!(
        "HTTP/1.1 {}\r\n{}Content-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status,
        header_factory(),
        mime_type,
        response.len()
    );
    let _ = stream.write_all(val.as_bytes());
    let _ = stream.write_all(response);
    let _ = stream.flush();
}
