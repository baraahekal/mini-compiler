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
    keywords: HashSet<String>,
    identifiers: HashSet<String>,
    operators: HashSet<String>,
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
    let keywords = Regex::new(r"\b(if|else|for|while|return)\b").unwrap();
    let identifiers = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let operators = Regex::new(r"(\+|-|\*|/|%|==|!=|<|>|<=|>=|\|\||&&)").unwrap();

    let mut tokens = Tokens {
        keywords: HashSet::new(),
        identifiers: HashSet::new(),
        operators: HashSet::new(),
    };

    for cap in keywords.captures_iter(&code.code) {
        tokens.keywords.insert(cap[0].to_string());
    }

    for cap in identifiers.captures_iter(&code.code) {
        if !tokens.keywords.contains(&cap[0]) {
            tokens.identifiers.insert(cap[0].to_string());
        }
    }

    for cap in operators.captures_iter(&code.code) {
        tokens.operators.insert(cap[0].to_string());
    }

    // Return tokens as JSON response
    Ok(warp::reply::json(&tokens))
}
