use std::io::Write;
use std::net::TcpStream;

pub fn send_response(status:u16, mut stream: TcpStream, response: &str) {
    let val = format!("HTTP/1.1 {}\r\n{}", status, response);
    let _ = stream.write_all(val.as_bytes());
    let _ = stream.flush();
}