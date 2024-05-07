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
}

#[tokio::main]
async fn main() {
    // Define route for API
    let api_route = warp::path("tokenize")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(tokenize_handler);

    let cors = warp::cors()
        .allow_origin("http://localhost:3000/")
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    // Apply CORS middleware to the API route
    let routes = api_route.with(cors);

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Compile regular expressions once
lazy_static! {
    static ref IDENTIFIERS: Regex = Regex::new(r"\b(int|float|string|double|bool|char)\b").unwrap();
    static ref SYMBOLS: Regex = Regex::new(r"(&&|\|\||[()+\-*/%=;{},|&><!])").unwrap();
    static ref RESERVED_WORDS: Regex = Regex::new(r"\b(for|while|return|end|if|do|break|continue)\b").unwrap();
    static ref VARIABLES: Regex = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    static ref LISTS: Regex = Regex::new(r"(\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\[\s*[a-zA-Z0-9_]*\s*\])\s*(=\s*)?\{([a-zA-Z0-9_,\s]*)\}").unwrap();
    static ref SINGLE_LINE_COMMENT: Regex = Regex::new(r"(?://[^\n]*)").unwrap();
    static ref MULTI_LINE_COMMENT: Regex = Regex::new(r"/\*(.|\n)*?\*/").unwrap();
}

async fn tokenize_handler(mut code: Code) -> Result<impl Reply, Rejection> {
    let mut tokens = Tokens {
        identifiers: HashSet::new(),
        symbols: HashSet::new(),
        reserved_words: HashSet::new(),
        variables: HashSet::new(),
        lists: HashMap::new(),
        comments: Vec::new(),
    };

    // Store and remove comments
    for cap in SINGLE_LINE_COMMENT.captures_iter(&code.code) {
        tokens.comments.push(cap[0].to_string());
    }
    for cap in MULTI_LINE_COMMENT.captures_iter(&code.code) {
        tokens.comments.push(cap[0].to_string());
    }

    code.code = SINGLE_LINE_COMMENT.replace_all(&code.code, "").to_string();
    code.code = MULTI_LINE_COMMENT.replace_all(&code.code, "").to_string();

    // Capture symbols
    for cap in SYMBOLS.captures_iter(&code.code) {
        let token = cap[0].to_string();
        tokens.symbols.insert(token);
    }

    // Capture identifier, reserved words, and variables in a single iteration
    for cap in VARIABLES.captures_iter(&code.code) {
        let token = cap[0].to_string();
        if IDENTIFIERS.is_match(&token) {
            tokens.identifiers.insert(token);
        } else if RESERVED_WORDS.is_match(&token) {
            tokens.reserved_words.insert(token);
        } else if VARIABLES.is_match(&token){
            tokens.variables.insert(token);
        }
    }

    // Capture lists
    for cap in LISTS.captures_iter(&code.code) {
        let list_declaration = cap[1].to_string();
        let list_initialization = cap[0].to_string();
        tokens.lists.insert(list_declaration, list_initialization);
    }

    Ok(warp::reply::json(&tokens))
}
