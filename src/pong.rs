use std::io::Write;
use std::net::TcpStream;

pub fn send_response(mut stream: TcpStream, response: &str) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}