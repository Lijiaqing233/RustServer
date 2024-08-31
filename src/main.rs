use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let _ip_address = "127.0.0.1:3001";
    
    let app = Router::new()
        .route("/", get(handler));
    
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
