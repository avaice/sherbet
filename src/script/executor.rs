use std::collections::HashMap;

pub struct Executor {
    variables: HashMap<String, String>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, src: &str) -> Option<String> {
        let tokens: Vec<&str> = src.split_whitespace().collect();

        // 空行はスキップ
        if tokens.is_empty() {
            return None;
        }

        if let Some(&first_token) = tokens.first() {
            match first_token {
                "print" => {
                    if tokens.len() <= 1 {
                        return Some("print command needs an argument".to_string());
                    } else {
                        return self.execute(&(tokens[1..].join(" ")));
                    }
                }
                "return" => {
                    if tokens.len() <= 1 {
                        return None;
                    } else {
                        return self.execute(&(tokens[1..].join(" ")));
                    }
                }
                "let" => {
                    if tokens.len() < 4 || tokens[2] != "=" {
                        return Some("let syntax: let variable_name = value".to_string());
                    } else {
                        let var_name = tokens[1].to_string();
                        let value = tokens[3..].join(" ");
                        self.variables.insert(var_name, value);
                    }
                }
                _ => {
                    // 変数代入の処理
                    if tokens.len() >= 3 && tokens[1] == "=" {
                        let var_name = tokens[0].to_string();
                        let value = tokens[2..].join(" ");
                        if self.variables.contains_key(&var_name) {
                            self.variables.insert(var_name, value);
                        } else {
                            return Some(format!("undefined variable: {}", var_name));
                        }
                    } else {
                        return Some("unknown command".to_string());
                    }
                }
            }
        }

        return None;
    }
}
