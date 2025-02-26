use std::{env, fs, net::TcpStream, path::Path};
use mime_guess::from_path;
use crate::{factory::plain_text_factory, pong::{send_binary_response, send_response}};

pub fn router(uri: &str, stream: TcpStream) {
    let static_dir = env::var("STATIC_DIR").unwrap_or("public".to_string());
    let static_dir = Path::new(&static_dir).canonicalize().unwrap();

    let file_path = if uri == "/" {
        static_dir.join("index.html")
    } else {
        static_dir.join(uri.trim_start_matches('/'))
    };

    if let Ok(canon_path) = file_path.canonicalize() {
        if canon_path.starts_with(&static_dir) {
            if let Ok(contents) = fs::read(&canon_path) {
                let mime_type = from_path(&canon_path).first_or_octet_stream();
                send_binary_response(200, stream, &contents, &mime_type);
            } else {
                let response = plain_text_factory("500 Internal Server Error");
                send_response(500, stream, &response);
            }
        } else {
            let response = plain_text_factory("403 Forbidden");
            send_response(403, stream, &response);
        }
    } else {
        let response = plain_text_factory("404 Not Found");
        send_response(404, stream, &response);
    }
}
