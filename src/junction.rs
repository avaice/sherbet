
use std::{env, fs, net::TcpStream, path::Path};

use mime_guess::from_path;

use crate::{factory::plain_text_factory, pong::{send_binary_response, send_response}};

pub fn router(uri: &str, stream: TcpStream) {
    let static_dir = env::var("STATIC_DIR").unwrap_or("public".to_string()); 
    let file_path = if uri == "/" {
        format!("{}/index.html", static_dir)
    } else {
        format!("{}{}", static_dir, uri)
    };

    if Path::new(&file_path).exists() {
        // ファイルをバイナリで読む
        let contents = fs::read(&file_path).unwrap();
        let mime_type = from_path(&file_path).first_or_octet_stream();
        send_binary_response(200, stream, &contents, &mime_type);
    } else {
        let response = plain_text_factory( "404 Not Found");
        send_response(404, stream, &response);
    }
}