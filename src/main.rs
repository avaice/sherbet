mod ping;
mod pong;
mod factory;

use std::env;
use crate::ping::handle_client;
use std::net::TcpListener;
use dotenvy::dotenv;

fn main() {

    // Load environment variables from .env file
    dotenv().ok(); 

    let http_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", http_port)).unwrap();
    println!("Listening on port {}", http_port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}
