use axum::{
    routing::get,
    Router,
};
use std::{env, net::SocketAddr};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Get port from Railway (or default to 8080 for local)
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    println!("🚀 Server running on http://{}", addr);

    // Serve static files from "static" folder
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
