use axum::{
    routing::get,
    Router,
    response::Html,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

async fn index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

#[tokio::main]
async fn main() {
    // Serve static files (css + images)
    let static_files = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", static_files);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
