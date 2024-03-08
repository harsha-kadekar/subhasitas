use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tokio;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "subhasitas=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app())
        .await
        .unwrap();

}


fn app () -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))

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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body, 
        http::{Request, StatusCode}
    };
    use http_body_util::BodyExt;
    use tower::util::ServiceExt;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn root_test() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello World!");
    }

    #[tokio::test]
    async fn hello_test() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder().uri("/hello").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "message": "Hello World!".to_string() }));

    }
    
}
