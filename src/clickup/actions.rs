use super::team::TeamId;

use serde::Serialize;

#[derive(Serialize, Clone, Hash)]
struct CreateTaskParameters {
    pub name: String,
    pub description: String,
}

/// Creates a clickup task
pub async fn create_task(
    team_id: impl Into<TeamId>,
    authorization: &str,
    name: &str,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://api.clickup.com/api/v2/team/{}/webhook",
        team_id.into().0
    );

    let params = CreateTaskParameters {
        name: String::from(name),
        description: String::from("generated task"),
    };

    client
        .post(url)
        .header(reqwest::header::AUTHORIZATION, authorization)
        .json(&params)
        .send()
        .await?
        .text()
        .await
}
