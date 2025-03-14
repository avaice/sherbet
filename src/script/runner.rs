pub fn runner(src: &[u8]) -> Option<String> {
    // read src as string
    let src = String::from_utf8_lossy(src);

    let parsed = src.split(';');

    // iterate over parsed src
    for line in parsed {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // 空行はスキップ
        if tokens.is_empty() {
            continue;
        }

        // match first token safely
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
                    continue;
                }
            }
        }
    }

    None
}
