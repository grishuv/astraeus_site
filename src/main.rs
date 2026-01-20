use axum::{
    routing::get,
    Router,
    response::Html,
};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    // Railway provides PORT
    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    let app = Router::new()
        .route("/", get(home));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Server running on http://{}", addr);

    // NEW AXUM 0.7 STYLE
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn home() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Astraeus Next Gen</title>
    <style>
        body {
            background:#0f172a;
            color:white;
            font-family:Arial;
            text-align:center;
            padding-top:100px;
        }
    </style>
</head>
<body>
    <h1>🚀 Astraeus Next Gen</h1>
    <p>Rust website successfully deployed on Railway!</p>
</body>
</html>
"#)
}
