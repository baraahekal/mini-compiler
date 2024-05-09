use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};

#[derive(Deserialize)]
struct Code {
    code: String,
}

#[derive(Serialize)]
struct Tokens {
    identifiers: HashSet<String>,
    symbols: HashSet<String>,
    reserved_words: HashSet<String>,
    variables: HashSet<String>,
    lists: HashMap<String, String>,
    comments: Vec<String>,
    literals: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    let api_route = warp::path("tokenize")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(tokenize_handler);

    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    let routes = api_route.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
 
fn process_comments(code: &str, tokens: &mut Tokens) -> String {
    let mut cleaned_code = String::new();
    let mut lines = code.lines();
    let mut in_multi_line_comment = false;

    while let Some(line) = lines.next() {
        if in_multi_line_comment {
            if let Some(end) = line.find("*/") {
                in_multi_line_comment = false;
                tokens.comments.push(line[..end+2].to_string());
                if end + 2 < line.len() {
                    cleaned_code.push_str(&line[end+2..]);
                    cleaned_code.push('\n');
                }
            } else {
                tokens.comments.push(line.to_string());
            }
        } else {
            if let Some(start) = line.find("/*") {
                in_multi_line_comment = true;
                cleaned_code.push_str(&line[..start]);
                cleaned_code.push('\n');
                if let Some(end) = line[start..].find("*/") {
                    in_multi_line_comment = false;
                    tokens.comments.push(line[start..start+end+2].to_string());
                    if start + end + 2 < line.len() {
                        cleaned_code.push_str(&line[start+end+2..]);
                        cleaned_code.push('\n');
                    }
                } else {
                    tokens.comments.push(line[start..].to_string());
                }
            } else if let Some(start) = line.find("//") {
                tokens.comments.push(line[start..].to_string());
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

fn process_literals(word: &str) -> Option<(&str, String)> {
    if word.starts_with('"') && word.ends_with('"') {
        Some((word.trim_matches('"'), "String".to_string()))
    } else if word.starts_with('\'') && word.ends_with('\'') && word.len() == 3 {
        Some((word, "Character".to_string()))
    } else if word == "true" || word == "false" {
        Some((word, "Boolean".to_string()))
    } else if word.parse::<i32>().is_ok() || word.parse::<f64>().is_ok() {
        Some((word, "Numeric".to_string()))
    } else {
        None
    }
}

fn process_symbols(code: &str, tokens: &mut Tokens) {
    let symbols: HashSet<char> = ['(', ')', '+', '-', '*', '/', '%', '=', ';', '{', '}', ',', '|', '&', '>', '<', '!'].iter().cloned().collect();

    let chars: Vec<char> = code.chars().collect();
    for i in 0..chars.len() {
        if (chars[i] == '&') && i + 1 < chars.len() && chars[i] == chars[i + 1] || (chars[i] == '|') && i + 1 < chars.len() && chars[i] == chars[i + 1] {
            tokens.symbols.insert(format!("{}{}", chars[i], chars[i]));
        }
        else if symbols.contains(&chars[i]) && i > 0 &&chars[i - 1] != chars[i] {
            tokens.symbols.insert(chars[i].to_string());
        }
    }
}

fn process_identifiers_and_reserved_words(word: &str) -> Option<(&str, String)> {
    let identifiers: HashSet<&str> = ["void", "int", "float", "string", "double", "bool", "char"].iter().cloned().collect();
    let reserved_words: HashSet<&str> = ["for", "while", "return", "end", "if", "do", "break", "continue"].iter().cloned().collect();

    if identifiers.contains(word) {
        Some((word, "Identifier".to_string()))
    } else if reserved_words.contains(word) {
        Some((word, "Reserved Word".to_string()))
    } else {
        None
    }
}

fn process_variables(word: &str) -> Option<(&str, String)> {
    if word.chars().all(|c| c.is_alphanumeric() || c == '_') && (word.chars().next().unwrap_or_default().is_alphabetic() || word.starts_with('_')) {
        Some((word, "Variable".to_string()))
    } else {
        None
    }
}

fn process_lists(code: &str, tokens: &mut Tokens) -> String {
    let mut cleaned_code = String::new();
    let mut lines = code.lines();

    while let Some(line) = lines.next() {
        if line.contains("[") && line.contains("]") && line.contains("{") && line.contains("}") {
            let parts: Vec<&str> = line.split("{").collect();
            if parts.len() == 2 {
                let list_declaration = parts[0].trim().trim_end_matches("=").trim().to_string();
                let list_elements: Vec<&str> = parts[1].trim_end_matches("}").trim_end_matches(";").trim().split(',').collect();
                let list_length = list_elements.len();
                let list_initialization = format!("{{{} (length: {})", parts[1].trim_end_matches("}").trim_end_matches(";").trim(), list_length);
                tokens.lists.insert(list_declaration, list_initialization);
            } else {
                cleaned_code.push_str(line);
                cleaned_code.push('\n');
            }
        } else {
            cleaned_code.push_str(line);
            cleaned_code.push('\n');
        }
    }

    cleaned_code
}    

async fn tokenize_handler(mut code: Code) -> Result<impl Reply, Rejection> {
    let mut tokens = Tokens {
        identifiers: HashSet::new(),
        symbols: HashSet::new(),
        reserved_words: HashSet::new(),
        variables: HashSet::new(),
        lists: HashMap::new(),
        comments: Vec::new(),
        literals: HashMap::new(),
    };

    code.code = process_comments(&code.code, &mut tokens);

    for word in code.code.split(|c: char| c.is_whitespace() || c == ';') {
        if let Some((literal, literal_type)) = process_literals(word) {
            tokens.literals.insert(literal.to_string(), literal_type);
        }
    }

    process_symbols(&code.code, &mut tokens);

    for word in code.code.split(|c: char| c.is_whitespace() || c == ';') {
        if let Some((token, token_type)) = process_identifiers_and_reserved_words(word) {
            match token_type.as_str() {
                "Identifier" => { let _ = tokens.identifiers.insert(token.to_string()); },
                "Reserved Word" => { let _ = tokens.reserved_words.insert(token.to_string()); },
                _ => (),
            }
        } else if let Some((token, _)) = process_variables(word) {
            let _ = tokens.variables.insert(token.to_string());
        }
    }
    
    code.code = process_lists(&code.code, &mut tokens);

    Ok(warp::reply::json(&tokens))
}