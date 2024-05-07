use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use regex::Regex;
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

async fn tokenize_handler(mut code: Code) -> Result<impl Reply, Rejection> {
    let identifiers = Regex::new(r"\b(int|float|string|double|bool|char)\b").unwrap();
    let symbols = Regex::new(r"(&&|\|\||[()+\-*/%=;{},|&><!])").unwrap();
    let reserved_words = Regex::new(r"\b(for|while|return|end|if|do|break|continue)\b").unwrap();
    let variables = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let lists = Regex::new(r"(\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\[\s*[a-zA-Z0-9_]*\s*\])\s*(=\s*)?\{([a-zA-Z0-9_,\s]*)\}").unwrap();
    let single_line_comment = Regex::new(r"(?://[^\n]*)").unwrap();
    let multi_line_comment = Regex::new(r"/\*(.|\n)*?\*/").unwrap();

    let mut tokens = Tokens {
        identifiers: HashSet::new(),
        symbols: HashSet::new(),
        reserved_words: HashSet::new(),
        variables: HashSet::new(),
        lists: HashMap::new(),
        comments: Vec::new(),
    };

    // Store comments
    for cap in single_line_comment.captures_iter(&code.code) {
        tokens.comments.push(cap[0].to_string());
    }

    for cap in multi_line_comment.captures_iter(&code.code) {
        tokens.comments.push(cap[0].to_string());
    }

    // Remove comments from code
    code.code = single_line_comment.replace_all(&code.code, "").to_string();
    code.code = multi_line_comment.replace_all(&code.code, "").to_string();

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

    for cap in lists.captures_iter(&code.code) {
        let list_declaration = cap[1].to_string();
        let list_initialization = cap[0].to_string();
        tokens.lists.insert(list_declaration, list_initialization);
    }

    // Return tokens as JSON file
    Ok(warp::reply::json(&tokens))
}
