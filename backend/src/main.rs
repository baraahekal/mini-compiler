use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
// use parser::Parser;
mod parser;
mod scanner;
mod token;

#[tokio::main]
async fn main() {
    let api_route = warp::path("tokenize")
        .and(warp::post())
        // .and(warp::body::json())
        .and_then(scanner::scanning_input_code);

    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    let routes = api_route.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}