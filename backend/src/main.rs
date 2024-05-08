use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;

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

lazy_static! {
    static ref IDENTIFIERS: Regex = Regex::new(r"\b(void|int|float|string|double|bool|char)\b").unwrap();
    static ref SYMBOLS: Regex = Regex::new(r"(&&|\|\||[()+\-*/%=;{},|&><!])").unwrap();
    static ref RESERVED_WORDS: Regex = Regex::new(r"\b(for|while|return|end|if|do|break|continue)\b").unwrap();
    static ref VARIABLES: Regex = Regex::new(r"\b[a-zA-Z_]\w*\b").unwrap();
    static ref LISTS: Regex = Regex::new(r"(\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\[\s*[a-zA-Z0-9_]*\s*\])\s*(=\s*)?\{([a-zA-Z0-9_,\s]*)\}").unwrap();
    static ref SINGLE_LINE_COMMENT: Regex = Regex::new(r"(?://[^\n]*)").unwrap();
    static ref MULTI_LINE_COMMENT: Regex = Regex::new(r"/\*(.|\n)*?\*/").unwrap();
    static ref NUMERIC_LITERAL: Regex = Regex::new(r"^\d+(\.\d+)?$").unwrap();
    static ref CHARACTER_LITERAL: Regex = Regex::new(r"^'.'$").unwrap();
    static ref STRING_LITERAL: Regex = Regex::new(r#"^".*"$"#).unwrap();
    static ref BOOLEAN_LITERAL: Regex = Regex::new(r"^(true|false)$").unwrap();
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

    for cap in SINGLE_LINE_COMMENT.captures_iter(&code.code) {
        tokens.comments.push(cap[0].to_string());
    }

    for cap in MULTI_LINE_COMMENT.captures_iter(&code.code) {
        tokens.comments.push(cap[0].to_string());
    }

    // Remove comments from code
    code.code = SINGLE_LINE_COMMENT.replace_all(&code.code, "").to_string();
    code.code = MULTI_LINE_COMMENT.replace_all(&code.code, "").to_string();

    for word in code.code.split(|c: char| c.is_whitespace() || c == ';') {
        if NUMERIC_LITERAL.is_match(word) {
            tokens.literals.insert(word.to_string(), "Numeric".to_string());
        } else if CHARACTER_LITERAL.is_match(word) {
            tokens.literals.insert(word.to_string(), "Character".to_string());
        } else if STRING_LITERAL.is_match(word) {
            let trimmed_word = word.trim_matches('"').to_string();
            tokens.literals.insert(trimmed_word.to_string(), "String".to_string());
        } else if BOOLEAN_LITERAL.is_match(word) {
            tokens.literals.insert(word.to_string(), "Boolean".to_string());
        }
    }

    for cap in SYMBOLS.captures_iter(&code.code) {
        let token = cap[0].to_string();
        tokens.symbols.insert(token);
    }

    for cap in VARIABLES.captures_iter(&code.code) {
        let token = cap[0].to_string();
        if IDENTIFIERS.is_match(&token) {
            tokens.identifiers.insert(token);
        } else if RESERVED_WORDS.is_match(&token) {
            tokens.reserved_words.insert(token);
        } else if !tokens.literals.contains_key(&token) {
            println!("{}", token);
            tokens.variables.insert(token);
        }
    }

    for cap in LISTS.captures_iter(&code.code) {
        let list_declaration = cap[1].to_string();
        let list_initialization = cap[0].to_string();
        tokens.lists.insert(list_declaration, list_initialization);
    }

    println!("{}", tokens.literals.len());

    Ok(warp::reply::json(&tokens))
}