use std::net::SocketAddr;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Railway provides PORT automatically
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Server running on http://{}", addr);

    let app = Router::new()
        .route("/", get(|| async { 
            "Astraeus Next Gen is live 🚀" 
        }));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
