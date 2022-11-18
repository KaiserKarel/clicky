use crate::MILESTONE_SPACES;

use super::auth::ClickupToken;
use super::list::ListId;

use super::task::{Space, Task, TaskId};
use serde::Serialize;

#[derive(Serialize, Clone, Hash)]
struct CreateTaskParameters {
    pub name: String,
    pub description: String,
    pub parent: String,
}

#[derive(Serialize, Clone, Hash)]
struct SetTaskParentParams<'a> {
    pub parent: &'a TaskId,
}

/// Creates a clickup task
pub async fn create_task(
    token: &ClickupToken,
    list: &ListId,
    name: &str,
) -> reqwest::Result<String> {
    let client = reqwest::Client::new();

    let url = format!("https://api.clickup.com/api/v2/list/{}/task", list.0);

    let params = CreateTaskParameters {
        name: String::from(name),
        description: String::from("generated task"),
        parent: String::from("36pnwzu"),
    };

    client
        .post(url)
        .header(reqwest::header::AUTHORIZATION, token.0)
        .json(&params)
        .send()
        .await?
        .text()
        .await
}

pub async fn get_task(token: &ClickupToken, id: &TaskId) -> reqwest::Result<Task> {
    let client = reqwest::Client::new();

    let url = format!("https://api.clickup.com/api/v2/task/{}", id.0);

    client
        .get(url)
        .header(reqwest::header::AUTHORIZATION, token.0)
        .send()
        .await?
        .json()
        .await
}

pub async fn set_task_parent(
    token: &ClickupToken,
    id: &TaskId,
    new_parent: &TaskId,
) -> reqwest::Result<Task> {
    let client = reqwest::Client::new();

    let url = format!("https://api.clickup.com/api/v2/task/{}", id.0);

    let params = SetTaskParentParams { parent: new_parent };

    client
        .put(url)
        .header(reqwest::header::AUTHORIZATION, token.0)
        .json(&params)
        .send()
        .await?
        .json()
        .await
}

pub async fn add_task_to_list(
    token: &ClickupToken,
    task: &TaskId,
    list: &ListId,
) -> reqwest::Result<String> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://api.clickup.com/api/v2/list/{}/task/{}",
        list.0, task.0
    );

    client
        .post(url)
        .header(reqwest::header::AUTHORIZATION, token.0)
        .send()
        .await?
        .text()
        .await
}

fn task_is_in_milestone_space(task: &Task) -> bool {
    MILESTONE_SPACES.contains(&task.space.id.as_str())
}

// pub async fn set_task_parent(authorization: &str

#[cfg(test)]
mod tests {

    use super::*;
    use crate::CLICKUP_TOKEN;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_get_task() {
        let res = get_task(&CLICKUP_TOKEN, &TaskId::from("36pnwzu"))
            .await
            .unwrap();
        dbg!(res);
    }

    #[tokio::test]
    #[traced_test]
    async fn task_that_is_in_milestone_space() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36pnwzu"))
            .await
            .unwrap();

        assert!(task_is_in_milestone_space(&task));
    }

    #[tokio::test]
    #[traced_test]
    async fn task_that_is_not_in_milestone_space() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w78wt"))
            .await
            .unwrap();

        assert!(!task_is_in_milestone_space(&task));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_task_with_parent() {
        let res = get_task(&CLICKUP_TOKEN, &TaskId::from("3vj469b"))
            .await
            .unwrap();
        dbg!(res);
    }

    #[tokio::test]
    #[traced_test]
    async fn can_set_task_parent() {
        let res = set_task_parent(
            &CLICKUP_TOKEN,
            &TaskId::from("36w79af"), // Task in picasso
            &TaskId::from("36pnwzu"), // v0 milestone
        )
        .await
        .unwrap();
        dbg!(res);
    }

    #[tokio::test]
    #[traced_test]
    async fn can_add_task_to_list() {
        add_task_to_list(
            &CLICKUP_TOKEN,
            &TaskId::from("36w79af"), // Task that was originally in picasso
            &ListId::from("188335750"), // picasso list
        )
        .await
        .unwrap();
    }
}
