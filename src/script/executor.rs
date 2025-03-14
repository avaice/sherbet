pub fn executor(src:&str) -> Option<String> {
    let tokens: Vec<&str> = src.split_whitespace().collect();

        // 空行はスキップ
        if tokens.is_empty() {
           return None;
        }

    if let Some(&first_token) = tokens.first() {
        match first_token {
            "print" => {
                if tokens.len() == 0 {
                    return Some("print command needs an argument".to_string());
                } else {
                    println!("{}", tokens[1..].join(" "));
                }
            }
            "return" => {
                if tokens.len() == 0 {
                    return None;
                } else {
                    return Some(tokens[1..].join(" "));
                }
            }
            _ => {
                return Some("unknown command".to_string());
            }
        }
    }

    return None;
}