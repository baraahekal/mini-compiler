use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Identifier,
    Symbol,
    ReservedWord,
    Variable,
    List,
    Comment,
    Literal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

#[derive(Deserialize)]
struct Code {
    code: String,
}

#[derive(Serialize, Clone)]
struct Tokens {
    tokens: Vec<Token>,
}

pub struct Scanner {
    code: String,
    tokens: Tokens,
}

impl Scanner {
    fn new(code: String) -> Self {
        Self {
            code,
            tokens: Tokens {
                tokens: Vec::new(),
            },
        }
    }


    fn process_comments(&mut self) -> String {
        let mut cleaned_code = String::new();
        let mut lines = self.code.lines();
        let mut in_multi_line_comment = false;

        while let Some(line) = lines.next() {
            if in_multi_line_comment {
                if let Some(end) = line.find("*/") {
                    in_multi_line_comment = false;
                    self.tokens.tokens.push(Token {
                        token_type: TokenType::Comment,
                        value: line[..end + 2].to_string(),
                    });

                    cleaned_code.push_str(&line[end + 2..]);
                    cleaned_code.push(' ');
                } else {
                    self.tokens.tokens.push(Token {
                        token_type: TokenType::Comment,
                        value: line.to_string(),
                    });
                }
            } else {
                if let Some(start) = line.find("/*") {
                    in_multi_line_comment = true;
                    cleaned_code.push_str(&line[..start]);
                    cleaned_code.push(' ');
                    if let Some(end) = line.find("*/") {
                        in_multi_line_comment = false;
                        self.tokens.tokens.push(Token {
                            token_type: TokenType::Comment,
                            value: line[start..end + 2].to_string(),
                        });

                        cleaned_code.push_str(&line[end + 2..]);
                        cleaned_code.push(' ');
                    } else {
                        self.tokens.tokens.push(Token {
                            token_type: TokenType::Comment,
                            value: line[start..].to_string(),
                        });
                    }
                } else if let Some(start) = line.find("//") {
                    self.tokens.tokens.push(Token {
                        token_type: TokenType::Comment,
                        value: line[start..].to_string(),
                    });
                    cleaned_code.push_str(&line[..start]);
                    cleaned_code.push(' ');
                } else {
                    cleaned_code.push_str(line);
                    cleaned_code.push('\n');
                }
            }
        }

        cleaned_code
    }

    fn process_literals(&mut self) {
        for word in self.code.split(|c: char| c.is_whitespace() || c == ';' || c == '=' || c == '(' || c == ')' || c == '{' || c == '}' || c == ',' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '|' || c == '&' || c == '>' || c == '<' || c == '!') {
            if word.starts_with('"') && word.ends_with('"') {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Literal,
                    value: word.trim_matches('"').to_string(),
                });
            } else if word.starts_with('\'') && word.ends_with('\'') && word.len() == 3 {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Literal,
                    value: word.to_string(),
                });
            } else if word == "true" || word == "false" {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Literal,
                    value: word.to_string(),
                });
            } else if word.parse::<i32>().is_ok() || word.parse::<f64>().is_ok() {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Literal,
                    value: word.to_string(),
                });
            }
        }
    }

    fn process_symbols(&mut self) {
        let symbols: Vec<char> = ['(', ')', '+', '-', '*', '/', '%', '=', ';', '{', '}', ',', '|', '&', '>', '<', '!'].to_vec();

        let chars: Vec<char> = self.code.chars().collect();
        for i in 0..chars.len() {
            if (chars[i] == '&') && i + 1 < chars.len() && chars[i] == chars[i + 1] || (chars[i] == '|') && i + 1 < chars.len() && chars[i] == chars[i + 1] {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Symbol,
                    value: format!("{}{}", chars[i], chars[i]),
                });
            }
            else if symbols.contains(&chars[i]) && i > 0 && chars[i - 1] != chars[i] {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Symbol,
                    value: chars[i].to_string(),
                });
            }
        }
    }

    fn process_identifiers_and_reserved_words(&mut self) {
        let identifiers: Vec<&str> = ["void", "int", "float", "string", "double", "bool", "char"].to_vec();
        let reserved_words: Vec<&str> = ["for", "while", "return", "end", "if", "do", "break", "continue"].to_vec();

        for word in self.code.split(|c: char| c.is_whitespace() || c == ';' || c == '=' || c == '(' || c == ')' || c == '{' || c == '}' || c == ',' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '|' || c == '&' || c == '>' || c == '<' || c == '!' || c == '"' || c == '\'') {
            if identifiers.contains(&word) {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Identifier,
                    value: word.to_string(),
                });
            } else if reserved_words.contains(&word) {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::ReservedWord,
                    value: word.to_string(),
                });
            }
        }
    }

      fn process_variables(&mut self) {
        #[cfg(test)]
        self.process_identifiers_and_reserved_words();
        use regex::Regex;
        for word in self.code.split(|c: char| c.is_whitespace() || c == ';' || c == '=' || c == '(' || c == ')' || c == '{' || c == '}' || c == ',' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '|' || c == '&' || c == '>' || c == '<' || c == '!' || c == '\'') {
            let variable_part = word.trim().to_string();

            // Check if the word is not an identifier or reserved word before classifying it as a variable
            if !self.tokens.tokens.iter().any(|token| token.value == variable_part && (matches!(token.token_type, TokenType::Identifier) || matches!(token.token_type, TokenType::ReservedWord))) && (Regex::new(r"^[a-zA-Z]").unwrap().is_match(&variable_part) || variable_part.starts_with('_')) {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Variable,
                    value: variable_part,
                });
            }
        }
    }

    fn process_lists(&mut self) {
        let mut lines = self.code.lines();

        while let Some(line) = lines.next() {
            if line.contains("[") && line.contains("]") && line.contains("{") && line.contains("}") {
                let parts: Vec<&str> = line.split("{").collect();

                    let list_declaration = parts[0].trim_end_matches("=").trim();
                    let list_initialization_part = parts[1].trim_end_matches("}").trim_end_matches(";").trim();
                    let list_length = if list_initialization_part.starts_with("}") {
                        0
                    } else {
                        list_initialization_part.split(',').count()
                    };
                    let list_initialization = format!("{{{} (length: {})", list_initialization_part, list_length);
                    self.tokens.tokens.push(Token {
                        token_type: TokenType::List,
                        value: format!("{} {}", list_declaration, list_initialization),
                    });

            }
        }
    }

    fn scan(&mut self) -> Tokens {
        self.code = self.process_comments();
        self.process_literals();
        self.process_symbols();
        self.process_identifiers_and_reserved_words();
        self.process_variables();
        self.process_lists();

        return self.tokens.clone()
    }
}

#[tokio::main]
async fn main() {
    let api_route = warp::path("tokenize")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(scanning_input_code);

    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    let routes = api_route.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn scanning_input_code(code: Code) -> Result<impl Reply, Rejection> {
    let mut scanner = Scanner::new(code.code);
    let tokens = scanner.scan();
    Ok(warp::reply::json(&tokens))
}
