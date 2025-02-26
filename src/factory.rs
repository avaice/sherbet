pub fn plain_text_factory(text: &str) -> String {
    return format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", text.len(), text)
}