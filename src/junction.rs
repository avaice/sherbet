use std::net::TcpStream;

use crate::{factory::plain_text_factory, pong::send_response};

pub fn router(uri: &str, stream: TcpStream) {
    if uri == "/" {
        let response = plain_text_factory("melted sherbet");
        send_response(200, stream, &response);
    } else if uri == "/hello" {
        let response = plain_text_factory("Greetings!");
        send_response(200, stream, &response);
    } else if uri == "/today" {
        let today = "Today is a great day!";
        let response = plain_text_factory(&today);
        send_response(200, stream, &response);
    } else{
        send_response(404, stream, "");
    }
}