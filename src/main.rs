use axum::Router;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Railway provides PORT automatically
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    println!("🚀 Server running on http://{}", addr);

    // Serve static website from /static folder
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"));

    // Correct way for Axum 0.7
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
