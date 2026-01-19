use axum::Router;
use std::{env, net::SocketAddr};
use tower_http::services::ServeDir;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Railway gives PORT automatically
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    println!("🚀 Server running on http://{}", addr);

    // Serve files from /static
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"));

    // Bind correctly for Railway
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
