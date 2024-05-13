use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use crate::token::{Token, TokenType, TokenGlobal};
// use crate::parser::Parser;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokens {
    tokens: Vec<Token>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code {
    pub code: String,
}

pub struct Scanner {
    code: String,
    tokens: Tokens,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(code: String) -> Self {
        Self {
            code,
            tokens: Tokens {
                tokens: Vec::new(),
            },
            line: 1,
            column: 0,
        }
    }

    fn process_comments(&mut self) -> String {
        let mut cleaned_code = String::new();
        let mut lines = self.code.lines();
        let mut in_multi_line_comment = false;
        let mut multi_line_comment = String::new();

        while let Some(line) = lines.next() {
            if in_multi_line_comment {
                if let Some(end) = line.find("*/") {
                    in_multi_line_comment = false;
                    multi_line_comment.push_str(&line[..end + 2]);
                    self.tokens.tokens.push(Token {
                        token_type: TokenType::Comment,
                        token_global: TokenGlobal::Comment,
                        lexeme: multi_line_comment.clone(),
                        line: self.line,
                        column: self.column,
                    });
                    multi_line_comment.clear();

                    cleaned_code.push_str(&line[end + 2..]);
                    cleaned_code.push(' ');
                } else {
                    multi_line_comment.push_str(line);
                    multi_line_comment.push('\n');
                }
            } else {
                if let Some(start) = line.find("/*") {
                    in_multi_line_comment = true;
                    cleaned_code.push_str(&line[..start]);
                    cleaned_code.push(' ');
                    multi_line_comment.push_str(&line[start..]);
                    multi_line_comment.push('\n');
                } else if let Some(start) = line.find("//") {
                    self.tokens.tokens.push(Token {
                        token_type: TokenType::Comment,
                        token_global: TokenGlobal::Comment,
                        lexeme: line[start..].to_string(),
                        line: self.line,
                        column: self.column,
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
                    token_type: TokenType::StringLiteral,
                    token_global: TokenGlobal::Literal,
                    lexeme: word.trim_matches('"').to_string(),
                    line: self.line,
                    column: self.column,
                });
                self.column += word.len();
            } else if word.starts_with('\'') && word.ends_with('\'') && word.len() == 3 {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::CharacterLiteral,
                    token_global: TokenGlobal::Literal,
                    lexeme: word.to_string(),
                    line: self.line,
                    column: self.column,
                });
                self.column += word.len();
            } else if word == "true" || word == "false" {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::BooleanLiteral,
                    token_global: TokenGlobal::Literal,
                    lexeme: word.to_string(),
                    line: self.line,
                    column: self.column,
                });
                self.column += word.len();
            } else if word.parse::<i32>().is_ok() {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::IntegerLiteral,
                    token_global: TokenGlobal::Literal,
                    lexeme: word.to_string(),
                    line: self.line,
                    column: self.column,
                });
                self.column += word.len();
            } else if word.parse::<f64>().is_ok() {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::FloatingLiteral,
                    token_global: TokenGlobal::Literal,
                    lexeme: word.to_string(),
                    line: self.line,
                    column: self.column,
                });
                self.column += word.len();
            }
            if word.contains('\n') {
                self.line += word.matches('\n').count();
                self.column = 0;
            }
        }
    }

    fn process_symbols(&mut self) {
        let symbols: Vec<char> = ['(', ')', '+', '-', '*', '/', '%', '=', ';', '{', '}', ',', '|', '&', '>', '<', '!'].to_vec();

        let chars: Vec<char> = self.code.chars().collect();
        for i in 0..chars.len() {
            if symbols.contains(&chars[i]) {
                let token_type = match chars[i] {
                    '(' => TokenType::OpenParen,
                    ')' => TokenType::CloseParen,
                    '+' => TokenType::Plus,
                    '-' => TokenType::Minus,
                    '*' => TokenType::Multiply,
                    '/' => TokenType::Divide,
                    '%' => TokenType::Modulo,
                    '=' => TokenType::Equal,
                    ';' => TokenType::Semicolon,
                    '{' => TokenType::OpenBrace,
                    '}' => TokenType::CloseBrace,
                    ',' => TokenType::Comma,
                    '|' => TokenType::BitwiseOr,
                    '&' => TokenType::BitwiseAnd,
                    '>' => TokenType::GreaterThan,
                    '<' => TokenType::LessThan,
                    '!' => TokenType::Exclamation,
                    _ => unreachable!(),
                };

                self.tokens.tokens.push(Token {
                    token_type,
                    token_global: TokenGlobal::Symbol,
                    lexeme: chars[i].to_string(),
                    line: self.line,
                    column: self.column,
                });
            }
        }
    }

    fn process_identifiers(&mut self) {
        let identifiers: Vec<&str> = ["void", "int", "float", "string", "double", "bool", "char"].to_vec();

        for word in self.code.split(|c: char| c.is_whitespace() || c == ';' || c == '=' || c == '(' || c == ')' || c == '{' || c == '}' || c == ',' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '|' || c == '&' || c == '>' || c == '<' || c == '!' || c == '"' || c == '\'') {
            if identifiers.contains(&word) {
                let token_type = match word {
                    "void" => TokenType::Void,
                    "int" => TokenType::Int,
                    "float" => TokenType::Float,
                    "string" => TokenType::String,
                    "double" => TokenType::Double,
                    "bool" => TokenType::Bool,
                    "char" => TokenType::Char,
                    _ => unreachable!(),
                };

                self.tokens.tokens.push(Token {
                    token_type,
                    token_global: TokenGlobal::Identifier,
                    lexeme: word.to_string(),
                    line: self.line,
                    column: self.column,
                });
            }
        }
    }

    fn process_reserved_words(&mut self) {
        let reserved_words: Vec<&str> = ["for", "while", "return", "end", "if", "do", "break", "continue"].to_vec();

        for word in self.code.split(|c: char| c.is_whitespace() || c == ';' || c == '=' || c == '(' || c == ')' || c == '{' || c == '}' || c == ',' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '|' || c == '&' || c == '>' || c == '<' || c == '!' || c == '"' || c == '\'') {
            if reserved_words.contains(&word) {
                let token_type = match word {
                    "for" => TokenType::For,
                    "while" => TokenType::While,
                    "return" => TokenType::Return,
                    "if" => TokenType::If,
                    "do" => TokenType::Do,
                    "break" => TokenType::Break,
                    "continue" => TokenType::Continue,
                    _ => unreachable!(),
                };

                self.tokens.tokens.push(Token {
                    token_type,
                    token_global: TokenGlobal::ReservedWord,
                    lexeme: word.to_string(),
                    line: self.line,
                    column: self.column,
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
            if !self.tokens.tokens.iter().any(|token| token.lexeme == variable_part && (matches!(token.token_global, TokenGlobal::Identifier) || matches!(token.token_global, TokenGlobal::ReservedWord))) && (Regex::new(r"^[a-zA-Z]").unwrap().is_match(&variable_part) || variable_part.starts_with('_')) {
                self.tokens.tokens.push(Token {
                    token_type: TokenType::Variable,
                    token_global: TokenGlobal::Variable,
                    lexeme: variable_part,
                    line: self.line,
                    column: self.column,
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
                    token_global: TokenGlobal::List,
                    lexeme: format!("{} {}", list_declaration, list_initialization),
                    line: self.line,
                    column: self.column,
                });

            }
        }
    }

    pub fn scan(&mut self) -> Tokens {
        self.code = self.process_comments();
        self.process_literals();
        self.process_symbols();
        self.process_identifiers();
        self.process_reserved_words();
        self.process_variables();
        self.process_lists();

        return self.tokens.clone()
    }
}

pub async fn scanning_input_code(code: Code) -> Result<impl Reply, Rejection> {
    let mut scanner = Scanner::new(code.code);
    let tokens = scanner.scan();
    Ok(warp::reply::json(&tokens))
}