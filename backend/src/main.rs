use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};

#[derive(Deserialize)]
struct Code {
    code: String,
}

#[derive(Serialize, Clone)]
struct Tokens {
    identifiers: HashSet<String>,
    symbols: HashSet<String>,
    reserved_words: HashSet<String>,
    variables: HashSet<String>,
    lists: HashMap<String, String>,
    comments: Vec<String>,
    literals: HashMap<String, String>,
}

struct Scanner {
    code: String,
    tokens: Tokens,
}

impl Scanner {
    fn new(code: String) -> Self {
        Self {
            code,
            tokens: Tokens {
                identifiers: HashSet::new(),
                symbols: HashSet::new(),
                reserved_words: HashSet::new(),
                variables: HashSet::new(),
                lists: HashMap::new(),
                comments: Vec::new(),
                literals: HashMap::new(),
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
                    self.tokens.comments.push(line[..end+2].to_string());
                    if end + 2 < line.len() {
                        cleaned_code.push_str(&line[end+2..]);
                        cleaned_code.push('\n');
                    }
                } else {
                    self.tokens.comments.push(line.to_string());
                }
            } else {
                if let Some(start) = line.find("/*") {
                    in_multi_line_comment = true;
                    cleaned_code.push_str(&line[..start]);
                    cleaned_code.push('\n');
                    if let Some(end) = line[start..].find("*/") {
                        in_multi_line_comment = false;
                        self.tokens.comments.push(line[start..start+end+2].to_string());
                        if start + end + 2 < line.len() {
                            cleaned_code.push_str(&line[start+end+2..]);
                            cleaned_code.push('\n');
                        }
                    } else {
                        self.tokens.comments.push(line[start..].to_string());
                    }
                } else if let Some(start) = line.find("//") {
                    self.tokens.comments.push(line[start..].to_string());
                    cleaned_code.push_str(&line[..start]);
                    cleaned_code.push('\n');
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
                self.tokens.literals.insert(word.trim_matches('"').to_string(), "String".to_string());
            } else if word.starts_with('\'') && word.ends_with('\'') && word.len() == 3 {
                self.tokens.literals.insert(word.to_string(), "Character".to_string());
            } else if word == "true" || word == "false" {
                self.tokens.literals.insert(word.to_string(), "Boolean".to_string());
            } else if word.parse::<i32>().is_ok() || word.parse::<f64>().is_ok() {
                self.tokens.literals.insert(word.to_string(), "Numeric".to_string());
            }
        }
    }

    fn process_symbols(&mut self) {
        let symbols: HashSet<char> = ['(', ')', '+', '-', '*', '/', '%', '=', ';', '{', '}', ',', '|', '&', '>', '<', '!'].iter().cloned().collect();

        let chars: Vec<char> = self.code.chars().collect();
        for i in 0..chars.len() {
            if (chars[i] == '&') && i + 1 < chars.len() && chars[i] == chars[i + 1] || (chars[i] == '|') && i + 1 < chars.len() && chars[i] == chars[i + 1] {
                self.tokens.symbols.insert(format!("{}{}", chars[i], chars[i]));
            }
            else if symbols.contains(&chars[i]) && i > 0 && chars[i - 1] != chars[i] {
                self.tokens.symbols.insert(chars[i].to_string());
            }
        }
    }

    fn process_identifiers_and_reserved_words(&mut self) {
        let identifiers: HashSet<&str> = ["void", "int", "float", "string", "double", "bool", "char"].iter().cloned().collect();
        let reserved_words: HashSet<&str> = ["for", "while", "return", "end", "if", "do", "break", "continue"].iter().cloned().collect();

        for word in self.code.split(|c: char| c.is_whitespace() || c == ';' || c == '=' || c == '(' || c == ')' || c == '{' || c == '}' || c == ',' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '|' || c == '&' || c == '>' || c == '<' || c == '!' || c == '"' || c == '\'') {
            if identifiers.contains(word) {
                let _ = self.tokens.identifiers.insert(word.to_string());
            } else if reserved_words.contains(word) {
                let _ = self.tokens.reserved_words.insert(word.to_string());
            }
        }
    }

    fn process_variables(&mut self) {
        use regex::Regex;
        for word in self.code.split(|c: char| c.is_whitespace() || c == ';' || c == '=' || c == '(' || c == ')' || c == '{' || c == '}' || c == ',' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '|' || c == '&' || c == '>' || c == '<' || c == '!' || c == '\'') {
            let mut variable_part = word.trim().to_string();

            let symbols: Vec<char> = ['(', ')', '+', '-', '*', '/', '%', '=', ';', '{', '}', ',', '|', '&', '>', '<', '!'].to_vec();
            for symbol in symbols {
                variable_part = variable_part.replace(symbol, " ");
            }

            let parts: Vec<&str> = variable_part.split(' ').collect();
            variable_part = parts[0].trim().to_string();

            // Check if the word is not an identifier or reserved word before classifying it as a variable // word.chars().next().unwrap_or_default().is_alphabetic()
            if !self.tokens.identifiers.contains(&variable_part) && !self.tokens.reserved_words.contains(&variable_part) && (Regex::new(r"^[a-zA-Z]").unwrap().is_match(&variable_part) || variable_part.starts_with('_')) {
                let _ = self.tokens.variables.insert(variable_part);
            }
        }
    }

    fn process_lists(&mut self) {
        let mut cleaned_code = String::new();
        let mut lines = self.code.lines();

        while let Some(line) = lines.next() {
            if line.contains("[") && line.contains("]") && line.contains("{") && line.contains("}") {
                let parts: Vec<&str> = line.split("{").collect();
                if parts.len() == 2 {
                    let list_declaration = parts[0].trim().trim_end_matches("=").trim().to_string();
                    let list_initialization_part = parts[1].trim_end_matches("}").trim_end_matches(";").trim();
                    let list_length = if list_initialization_part.starts_with("}") {
                        0
                    } else {
                        list_initialization_part.split(',').count()
                    };
                    let list_initialization = format!("{{{} (length: {})", list_initialization_part, list_length);
                    self.tokens.lists.insert(list_declaration, list_initialization);
                } else {
                    cleaned_code.push_str(line);
                    cleaned_code.push('\n');
                }
            } else {
                cleaned_code.push_str(line);
                cleaned_code.push('\n');
            }
        }

        self.code = cleaned_code;
    }

    fn scan(&mut self) -> Tokens {
        self.code = self.process_comments();
        self.process_literals();
        self.process_symbols();
        self.process_identifiers_and_reserved_words();
        self.process_variables();
        self.process_lists();

        self.tokens.clone()
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