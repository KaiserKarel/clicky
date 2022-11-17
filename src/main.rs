mod clickup;
mod config;
mod events;
mod github;

use axum::{
    extract::Path,
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

use crate::events::Event;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/webhook/:webhook_id", post(webhook));

    tokio::task::spawn(async {
        use crate::clickup::webhooks::{events::Event, request};
        let response = request::create_webhook(
            20465559,
            ("https://clicky.fly.dev/webhook/clickup_id", Event::all()),
            String::from("pk_32525039_H1P8KP2ZGWXFZCCB8OPQM4KVI587COBF"),
        )
        .await
        .expect("creating a webhook should work");

        tracing::info!("created webhook: {:?}", response)
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn webhook(Path(webhook_id): Path<String>, payload: bytes::Bytes) -> impl IntoResponse {
    let event = Event::new(payload, webhook_id);
    tracing::info!("received webhook event: {:?}", event);
    StatusCode::OK
}
