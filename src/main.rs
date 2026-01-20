use axum::{
    routing::get_service,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Serve files from ./static
    let app = Router::new().nest_service(
        "/",
        get_service(ServeDir::new("static")).handle_error(|error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error serving file: {}", error),
            )
        }),
    );

    // Railway / local port
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Server running on http://{}", addr);

    // NEW Axum 0.7 way
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
