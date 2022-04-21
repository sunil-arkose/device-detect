use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use user_agent_parser::UserAgentParser;

#[derive(Deserialize, Debug)]
struct Fingerprint {
    user_agent: String,
}

#[derive(Serialize)]
struct FingerprintResponse {
    product_name: Option<String>,
    product_major: Option<String>,
    product_minor: Option<String>,
    product_patch: Option<String>,
}

#[allow(unused_variables)]
#[axum_macros::debug_handler]
async fn handle_fingerprint(
    Json(Fingerprint { user_agent }): Json<Fingerprint>,
) -> Json<FingerprintResponse> {
    let parser = UserAgentParser::from_path("regexes.yaml").unwrap();

    let product = parser.parse_product(&user_agent);

    println!("{}", user_agent);

    Json(FingerprintResponse {
        product_name: product.name.map(|s| s.to_string()),
        product_major: product.major.map(|s| s.to_string()),
        product_minor: product.minor.map(|s| s.to_string()),
        product_patch: product.patch.map(|s| s.to_string()),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/fingerprint", post(handle_fingerprint));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
