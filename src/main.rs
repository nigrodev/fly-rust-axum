use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {

    // Build our application with some example routes
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello));

    // Run it with hyper
    let addr : SocketAddr = "[::]:8080".parse().unwrap(); // Listen on port 8080 on all addresses IPv6 and IPv4 for fly.io
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --
// Basic handlers that responds with a static string
// --

async fn root() -> &'static str {
    "Go to /hello"
}

async fn hello() -> &'static str {
    "Hello, World!"
}