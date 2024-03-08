use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tokio;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();

}


async fn root() -> &'static str {
    "Hello World!"
}

async fn hello() -> (StatusCode, Json<Hello>) {
    let hello = Hello {
        message: "Hello World!".to_string(),
    };

    (StatusCode::OK, Json(hello))
}

#[derive(Serialize)]
struct Hello {
    message: String,
}
