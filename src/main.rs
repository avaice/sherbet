mod ping;
mod pong;
mod factory;
mod junction;
mod script;

use std::env;
use crate::ping::handle_client;
use std::net::TcpListener;
use dotenvy::dotenv;

pub static SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");


fn main() {

    // Load environment variables from .env file
    dotenv().ok(); 

    let http_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", http_port)).unwrap();
    println!("Listening on port {}", http_port);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}
