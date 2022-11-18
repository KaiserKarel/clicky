mod clickup;
mod config;
mod github;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use clickup::{auth::ClickupToken, task::TaskId};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::clickup::{list::ListId, team::TeamId};
use uuid::Uuid;

pub const TEAM_ID: TeamId = TeamId(20131398);
pub const CLICKUP_WEBHOOK: &str = "https://clickity.fly.dev/webhook/clickup_id";
pub const CLICKUP_TOKEN: ClickupToken =
    ClickupToken("pk_38221385_ZO414SRT0JWLDX77FHFNLCJE0LRR9ELN");

/// All spaces for which milestone management is enabled
/// TODO: make nicer structure instead of &str slice
pub const MILESTONE_SPACES: [&str; 1] = ["32279886"];
/// The milestone list for each milestone space, matches by index
/// TODO: make nicer structure instead of &str slice
pub const MILESTONE_LISTS: [&str; 1] = ["188335476"];

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
            request::create_webhook(&CLICKUP_TOKEN, TEAM_ID, (CLICKUP_WEBHOOK, Event::all()))
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Event {
    pub task_id: TaskId,
}

async fn webhook(Path(_): Path<String>, payload: bytes::Bytes) -> impl IntoResponse {
    use clickup::actions::{get_task, make_task_subtask_of_milestone_task_if_needed};

    let Ok(event) = serde_json::from_slice::<Event>(&payload) else {
        tracing::error!("Invalid payload received");
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let Ok(task) = get_task(&CLICKUP_TOKEN, &event.task_id).await else {
        tracing::error!("Error getting task from clickup");
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    match make_task_subtask_of_milestone_task_if_needed(&CLICKUP_TOKEN, &task).await {
        Ok(_) => {
            tracing::info!("Successfully made task subtask of milestone {:?}", task);
            StatusCode::OK
        }
        Err(err) => {
            tracing::error!("Error making task subtask of milestone {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn create() -> String {
    use crate::clickup::actions::create_task;

    let name = format!("Generated task {}", Uuid::new_v4());
    let list = ListId::from("188335750");
    let res = create_task(&CLICKUP_TOKEN, &list, &name).await;

    match res {
        Ok(r) => format!("Task {name} created with res {r}"),
        Err(e) => format!("Error creating task: {e}"),
    }
}
