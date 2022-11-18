use super::list::ListId;

use super::task::Task;
use serde::Serialize;

#[derive(Serialize, Clone, Hash)]
struct CreateTaskParameters {
    pub name: String,
    pub description: String,
    pub parent: String,
}

const LIST_ID: ListId = ListId(188335476);

/// Creates a clickup task
pub async fn create_task(authorization: &str, name: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let url = format!("https://api.clickup.com/api/v2/list/{}/task", LIST_ID.0);

    let params = CreateTaskParameters {
        name: String::from(name),
        description: String::from("generated task"),
        parent: String::from("36pnwzu"),
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

pub async fn get_task(authorization: &str, id: &str) -> reqwest::Result<Task> {
    let client = reqwest::Client::new();

    let url = format!("https://api.clickup.com/api/v2/task/{}", id);

    client
        .get(url)
        .header(reqwest::header::AUTHORIZATION, authorization)
        .send()
        .await?
        .json()
        .await
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::CLICKUP_TOKEN;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_get_task() {
        let res = get_task(CLICKUP_TOKEN, "36pnwzu").await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_task_with_parent() {
        let res = get_task(CLICKUP_TOKEN, "3vj469b").await.unwrap();
        dbg!(res);
    }
}
