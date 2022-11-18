mod clickup;
mod config;
mod events;
mod github;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

use crate::clickup::team::TeamId;
use crate::events::Event;
use uuid::Uuid;

pub const TEAM_ID: TeamId = TeamId(20131398);
pub const CLICKUP_WEBHOOK: &str = "https://clickity.fly.dev/webhook/clickup_id";
pub const CLICKUP_TOKEN: &str = "pk_38221385_ZO414SRT0JWLDX77FHFNLCJE0LRR9ELN";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/create", get(create))
        .route("/webhook/:webhook_id", post(webhook));

    tokio::task::spawn(async {
        use crate::clickup::webhooks::{events::Event, request};
        let response =
            request::create_webhook(TEAM_ID, (CLICKUP_WEBHOOK, Event::all()), CLICKUP_TOKEN)
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

async fn create() -> String {
    use crate::clickup::actions::create_task;

    let name = format!("Generated task {}", Uuid::new_v4());
    let res = create_task(CLICKUP_TOKEN, &name).await;

    match res {
        Ok(r) => format!("Task {name} created with res {r}"),
        Err(e) => format!("Error creating task: {e}"),
    }
}
