use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
struct Code {
    code: String,
}

#[derive(Serialize)]
struct Tokens {
    identifiers: HashSet<String>,
    symbols: HashSet<String>,
    reserved_words: HashSet<String>,
    variables: HashSet<String>,
}

#[tokio::main]
async fn main() {
    // Define route for API
    let api_route = warp::path("tokenize")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(tokenize_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    // Apply CORS middleware to the API route
    let routes = api_route.with(cors);

    // Start Warp server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn tokenize_handler(code: Code) -> Result<impl Reply, Rejection> {
    let identifiers = Regex::new(r"\b(int|float|string|double|bool|char)\b").unwrap();
    let symbols = Regex::new(r"[()+\-*/%=;{},|&><!]+").unwrap();
    let reserved_words = Regex::new(r"\b(for|while|return|end|if|do|break|continue)\b").unwrap();
    let variables = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();

    let mut tokens = Tokens {
        identifiers: HashSet::new(),
        symbols: HashSet::new(),
        reserved_words: HashSet::new(),
        variables: HashSet::new(),
    };

    for cap in identifiers.captures_iter(&code.code) {
        tokens.identifiers.insert(cap[0].to_string());
    }

    for cap in symbols.captures_iter(&code.code) {
        tokens.symbols.insert(cap[0].to_string());
    }

    for cap in reserved_words.captures_iter(&code.code) {
        tokens.reserved_words.insert(cap[0].to_string());
    }

    for cap in variables.captures_iter(&code.code) {
        let variable_name = cap[0].to_string();
        if !tokens.identifiers.contains(&variable_name) && !tokens.reserved_words.contains(&variable_name) {
            tokens.variables.insert(variable_name);
        }
    }

    // Return tokens as JSON response
    Ok(warp::reply::json(&tokens))
}
