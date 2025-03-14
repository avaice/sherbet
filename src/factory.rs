use crate::SERVER_VERSION;

pub fn header_factory() -> String {
    let version = SERVER_VERSION;
    return format!("Server: sharbet/{}\r\n", version);
}

pub fn plain_text_factory(text: &str) -> String {
    return format!("HTTP/1.1 200 OK\r\n{}Content-Type: text/plain; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}", header_factory(), text.len(), text)
}

pub fn json_factory(text: &str) -> String {
    return format!("HTTP/1.1 200 OK\r\n{}Content-Type: application/json; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}", header_factory(), text.len(), text)
}