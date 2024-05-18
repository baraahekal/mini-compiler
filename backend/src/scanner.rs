use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use crate::token::{Token, TokenType, TokenGlobal};
use crate::parser::Parser;
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
    fn split_into_tokens_with_positions(&self, split_chars: &[char]) -> Vec<(String, usize)> {
        let mut tokens = Vec::new();
        let mut token = String::new();
        let mut in_string_literal = false;
        let mut in_brackets = false;
        let mut in_braces = false;
        let mut column = 0;

        for c in self.code.chars() {
            if c == '"' {
                in_string_literal = !in_string_literal;
            } else if c == '[' {
                in_brackets = true;
            } else if c == ']' {
                in_brackets = false;
            } else if c == '{' {
                in_braces = true;
            } else if c == '}' {
                in_braces = false;
            }

            if in_string_literal || in_brackets || in_braces || !split_chars.contains(&c) {
                token.push(c);
            } else if !token.is_empty() {
                tokens.push((token.clone(), column - token.len() ));
                token.clear();
            }

            column += 1;
        }

        if !token.is_empty() {
            let token_len = token.len();
            tokens.push((token.clone(), column - token_len )); // +1 because column is 0-indexed
        }
        tokens
    }

    fn process_comments(&mut self) -> String {
        let mut cleaned_code = String::new();
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut in_multi_line_comment = false;
        let mut multi_line_comment = String::new();

        for line in lines {
            if in_multi_line_comment {
                if let Some(end) = line.find("*/") {
                    in_multi_line_comment = false;
                    multi_line_comment.push_str(&line[..end + 2]);
                    multi_line_comment.clear();

                    cleaned_code.push_str(&" ".repeat(end + 2));
                    cleaned_code.push_str(&line[end + 2..]);
                } else {
                    multi_line_comment.push_str(line);
                    multi_line_comment.push('\n');
                    cleaned_code.push_str(&" ".repeat(line.len()));
                }
            } else {
                if let Some(start) = line.find("/*") {
                    in_multi_line_comment = true;
                    cleaned_code.push_str(&line[..start]);
                    cleaned_code.push_str(&" ".repeat(line.len() - start));
                    multi_line_comment.push_str(&line[start..]);
                    multi_line_comment.push('\n');
                } else if let Some(start) = line.find("//") {
                    cleaned_code.push_str(&line[..start]);
                    cleaned_code.push_str(&" ".repeat(line.len() - start));
                } else {
                    cleaned_code.push_str(line);
                }
            }
            cleaned_code.push('\n');
        }

        cleaned_code
    }

    fn process_literals(&mut self, potential_token: &str,  original_line: usize, original_column: usize) -> bool {
        if potential_token.starts_with('"') && potential_token.ends_with('"') {
            self.tokens.tokens.push(Token {
                token_type: TokenType::StringLiteral,
                token_global: TokenGlobal::Literal,
                lexeme: potential_token.trim_matches('"').to_string(),
                line: self.line,
                column: self.column,
                original_line:original_line,
                original_column:original_column

            });
            self.column += potential_token.len();
            return true;
        } else if potential_token.starts_with('\'') && potential_token.ends_with('\'') && potential_token.len() == 3 {
            self.tokens.tokens.push(Token {
                token_type: TokenType::CharacterLiteral,
                token_global: TokenGlobal::Literal,
                lexeme: potential_token.to_string(),
                line: self.line,
                column: self.column,
                original_line:original_line,
                original_column:original_column
            });
            self.column += potential_token.len();
            return true;
        } else if potential_token == "true" || potential_token == "false" {
            self.tokens.tokens.push(Token {
                token_type: TokenType::BooleanLiteral,
                token_global: TokenGlobal::Literal,
                lexeme: potential_token.to_string(),
                line: self.line,
                column: self.column,
                original_line:original_line,
                original_column:original_column
            });
            self.column += potential_token.len();
            return true;
        } else if potential_token.parse::<i32>().is_ok() {
            self.tokens.tokens.push(Token {
                token_type: TokenType::IntegerLiteral,
                token_global: TokenGlobal::Literal,
                lexeme: potential_token.to_string(),
                line: self.line,
                column: self.column,
                original_line:original_line,
                original_column:original_column
            });
            self.column += potential_token.len();
            return true;
        } else if potential_token.parse::<f64>().is_ok() {
            self.tokens.tokens.push(Token {
                token_type: TokenType::FloatingLiteral,
                token_global: TokenGlobal::Literal,
                lexeme: potential_token.to_string(),
                line: self.line,
                column: self.column,
                original_line:original_line,
                original_column:original_column
            });
            self.column += potential_token.len();
            return true;
        }
        return false;
    }

    fn process_symbols(&mut self, potential_token: &str ,original_line: usize, original_column: usize) -> bool {
        let symbols: Vec<&str> = ["(", ")", "+", "-", "*", "/", "%", "=", ";", ":", "{", "}", ",", "|", "&", ">", "<", "!", "[", "]"].to_vec();
        let mut found = false;
        for symbol in &symbols {
            if potential_token.contains(symbol) {
                let token_type = match *symbol {
                    "(" => TokenType::OpenParen,
                    ")" => TokenType::CloseParen,
                    "+" => TokenType::Plus,
                    "-" => TokenType::Minus,
                    "*" => TokenType::Multiply,
                    "/" => TokenType::Divide,
                    "%" => TokenType::Modulo,
                    "=" => TokenType::Assignment,
                    ";" => TokenType::Semicolon,
                    ":" => TokenType::Colon,
                    "{" => TokenType::OpenBrace,
                    "}" => TokenType::CloseBrace,
                    "," => TokenType::Comma,
                    "|" => TokenType::BitwiseOr,
                    "&" => TokenType::BitwiseAnd,
                    ">" => TokenType::GreaterThan,
                    "<" => TokenType::LessThan,
                    "!" => TokenType::Exclamation,
                    "[" => TokenType::OpenBracket,
                    "]" => TokenType::CloseBracket,

                    _ => unreachable!(),
                };

                self.tokens.tokens.push(Token {
                    token_type,
                    token_global: TokenGlobal::Symbol,
                    lexeme: symbol.to_string(),
                    line: self.line,
                    column: self.column,
                    original_line:original_line,
                    original_column:original_column
                });
                self.column += symbol.len();
                found = true;
            }
        }
        found
    }


    fn process_identifiers(&mut self, potential_token: &str, original_line: usize, original_column: usize) -> bool {
        let identifiers: Vec<&str> = ["void", "int", "float", "string", "double", "bool", "char"].to_vec();

        if identifiers.contains(&potential_token) {
            let token_type = match potential_token {
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
                lexeme: potential_token.to_string(),
                line: self.line,
                column: self.column,
                original_line:original_line,
                original_column:original_column
            });
            self.column += potential_token.len();
            return true;
        }
        false
    }

    fn process_reserved_words(&mut self, potential_token: &str, original_line: usize, original_column: usize) -> bool {
        let reserved_words: Vec<&str> = ["for", "while", "return", "end", "if", "do", "break", "switch", "case", "continue", "else"].to_vec();
        let potential_token = potential_token.trim(); // Trim the whitespace

        for reserved_word in &reserved_words {
            if potential_token.starts_with(reserved_word) {
                let token_type = match *reserved_word {
                    "for" => TokenType::For,
                    "while" => TokenType::While,
                    "return" => TokenType::Return,
                    "if" => TokenType::If,
                    "do" => TokenType::Do,
                    "break" => TokenType::Break,
                    "continue" => TokenType::Continue,
                    "else" => TokenType::Else,
                    "switch" => TokenType::Switch,
                    "case" => TokenType::Case,

                    _ => unreachable!(),
                };

                self.tokens.tokens.push(Token {
                    token_type,
                    token_global: TokenGlobal::ReservedWord,
                    lexeme: reserved_word.to_string(),
                    line: self.line,
                    column: self.column,
                    original_line:original_line,
                    original_column:original_column
                });
                self.column += reserved_word.len();
                return true;
            }
        }
        false
    }

    fn process_variables(&mut self, potential_token: &str, original_line: usize, original_column: usize) -> bool {
        // Check if the word is not an identifier or reserved word before classifying it as a variable
        if !self.tokens.tokens.iter().any(|token| token.lexeme == potential_token && (matches!(token.token_global, TokenGlobal::Identifier) || matches!(token.token_global, TokenGlobal::ReservedWord))) && (Regex::new(r"^[a-zA-Z]").unwrap().is_match(potential_token) || potential_token.starts_with('_')) {
            self.tokens.tokens.push(Token {
                token_type: TokenType::Variable,
                token_global: TokenGlobal::Variable,
                lexeme: potential_token.to_string(),
                line: self.line,
                column: self.column,
                original_line:original_line,
                original_column:original_column
            });
            self.column += potential_token.len();
            return true;
        }
        false
    }


    fn process_lists(&mut self, line: &str ,original_line: usize, original_column: usize) -> bool {
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
                lexeme: format!("{}  {}", list_declaration, list_initialization),
                line: self.line,
                column: self.column,
                original_line:original_line,
                original_column:original_column
            });
            self.column += line.len();
            return true;
        }
        false
    }

    pub fn scan(&mut self) -> Tokens {
        self.code = self.process_comments();
        self.line = 1;
        let lines: Vec<String> = self.code.split('\n').map(|s| s.to_string()).collect();
        for line in &lines {

            // Add whitespace around equal sign, semicolon, parentheses, and braces
            let line = line.replace("=", " = ")
                .replace("+", " + ")
                .replace("-", " - ")
                .replace("*", " * ")
                .replace("/", " / ")
                .replace("%", " % ")
                .replace("++", " ++ ")
                .replace("--", " -- ")
                .replace("==", " == ")
                .replace("!=", " != ")
                .replace("<", " < ")
                .replace("<=", " <= ")
                .replace(">", " > ")
                .replace(">=", " >= ")
                .replace("&&", " && ")
                .replace("||", " || ")
                .replace("!", " ! ")
                .replace("&", " & ")
                .replace("|", " | ")
                .replace("^", " ^ ")
                .replace("~", " ~ ")
                .replace("<<", " << ")
                .replace(">>", " >> ")
                .replace("?", " ? ")
                .replace(":", " : ")
                .replace(";", " ; ")
                .replace(".", " . ")
                .replace("::", " :: ")
                .replace("[", " [ ")
                .replace("]", " ] ")
                .replace("(", " ( ")
                .replace(")", " ) ")
                .replace("{", " { ")
                .replace("}", " } ")
                .replace(",", " , ");
            self.code = line.clone();
            let original_lineL = self.line;
            let original_columnL = self.column;
            if self.process_lists(&line , original_lineL, original_columnL) {
                continue;
            }
            let potential_tokens: Vec<(String, usize)> = self.split_into_tokens_with_positions(&[' ']);
            for (potential_token, position) in potential_tokens {

                let original_token = potential_token.clone();
                let original_line = self.line;

                let mut original_column = self.column + position;
                if original_column > 0 {
                    original_column -= 1;
                }

                let potential_token = potential_token.replace(" ", "");


                if self.process_literals(&potential_token , original_line , original_column) {
                    continue;
                }
                if self.process_symbols(&potential_token ,original_line, original_column) {
                    continue;
                }
                if self.process_identifiers(&potential_token, original_line, original_column) {
                    continue;
                }
                if self.process_reserved_words(&potential_token,original_line, original_column) {
                    continue;
                }
                if self.process_variables(&potential_token,original_line, original_column) {
                    continue;
                }

            }
            self.line += 1;
            self.column = 0;
        }
        self.tokens.clone()
    }
}

pub async fn scanning_input_code(code: Code) -> Result<impl Reply, Rejection> {
    let mut scanner = Scanner::new(code.code);
    let tokens = scanner.scan();

    for token in &tokens.tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens.tokens);
    match parser.parse_program() {
        Ok(_) => {
            println!("Entered Ok match arm");
            Ok(warp::reply::json(&"No errors found."))
        },
        Err(errors) => {
            println!("{:?}", errors);
            Ok(warp::reply::json(&errors))
        },
    }
}