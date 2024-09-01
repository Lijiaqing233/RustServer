use axum::{response::Html, routing::get, Router};
use axum::extract::Path;
use axum::extract::Query;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let _ip_address = "127.0.0.1:3001";
    
    let app = Router::new()
        .route("/", get(handler))
        .route("/book/:id", get(path_extract))
        .route("/book", get(query_extract));
    
    let listener = tokio::net::TcpListener::bind(_ip_address)
        .await
        .unwrap();
    
    println!("listener: {0}", _ip_address);
    
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Test Rust!!!</h1>")
}

async fn path_extract(
    Path(id): Path<u32>,
) -> Html<String> {
    Html(format!("Hello, {id}！"))
}

async fn query_extract(
    Query(params): Query<HashMap<String, String>>
) -> Html<String> {
    Html(format!("{params:#?}"))
}
