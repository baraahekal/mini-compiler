use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::collections::{HashSet, HashMap};

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
    lists: HashMap<String, Vec<String>>,
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
    let lists = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*\[[^\[\]]*\][^\[\]]*").unwrap();
    let single_line_comment = Regex::new(r"(?://[^\n]*)").unwrap();
    let multi_line_comment = Regex::new(r"/\*(.|\n)*?\*/").unwrap();

    // Remove comments from code
    let cleaned_code = single_line_comment.replace_all(&code.code, "").to_string();
    let cleaned_code = multi_line_comment.replace_all(&cleaned_code, "").to_string();

    let mut tokens = Tokens {
        identifiers: HashSet::new(),
        symbols: HashSet::new(),
        reserved_words: HashSet::new(),
        variables: HashSet::new(),
        lists: HashMap::new(),
        comments: Vec::new(),
    };

    for cap in identifiers.captures_iter(&cleaned_code) {
        tokens.identifiers.insert(cap[0].to_string());
    }

    for cap in symbols.captures_iter(&cleaned_code) {
        tokens.symbols.insert(cap[0].to_string());
    }

    for cap in reserved_words.captures_iter(&cleaned_code) {
        tokens.reserved_words.insert(cap[0].to_string());
    }

    for cap in variables.captures_iter(&cleaned_code) {
        let variable_name = cap[0].to_string();
        if !tokens.identifiers.contains(&variable_name) && !tokens.reserved_words.contains(&variable_name) {
            tokens.variables.insert(variable_name);
        }
    }

    for cap in lists.captures_iter(&cleaned_code) {
        let list_name = cap[0].split('[').next().unwrap().to_string();
        let list_items = cap[0].trim_start_matches(|c| c == '[').trim_end_matches(|c| c == ']').split(',').map(|s| s.trim().to_string()).collect(); // Extract list items
        tokens.lists.insert(list_name, list_items);
    }

    // Store comments
    for cap in single_line_comment.captures_iter(&code.code) {
        tokens.comments.push(cap[0].to_string());
    }

    for cap in multi_line_comment.captures_iter(&code.code) {
        tokens.comments.push(cap[0].to_string());
    }

    // Return tokens as JSON response
    Ok(warp::reply::json(&tokens))
}
