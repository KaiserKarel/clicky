use crate::{MILESTONE_LISTS, MILESTONE_SPACES};

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

fn task_is_in_milestone_list(task: &Task) -> bool {
    let space_milestone_list = MILESTONE_SPACES
        .iter()
        .position(|&space| space == task.space.id.as_str())
        .map(|space_pos| MILESTONE_LISTS.get(space_pos))
        .flatten();

    match space_milestone_list {
        Some(list) => &task.list.id.0.as_str() == list,
        None => false,
    }
}

async fn task_is_transitive_subtask_of_milestone_task(
    token: &ClickupToken,
    task: &Task,
) -> reqwest::Result<bool> {
    if !task_is_in_milestone_space(task) {
        return Ok(false);
    }

    let mut current_task = task.clone();

    while let Some(parent_id) = &current_task.parent {
        let parent_task = get_task(token, parent_id).await?;
        if task_is_in_milestone_list(&parent_task) {
            return Ok(true);
        }
        current_task = parent_task;
    }

    Ok(false)
}

/// Gets the corresponding milestone destionation based on the custom `Milestone` field.
/// TODO: Make easily configurable
fn milestone_destionation_for_task(task: &Task) -> Option<TaskId> {
    task.custom_fields
        .iter()
        .find(|cf| cf.name == "Milestone")
        .and_then(|cf| cf.value.clone())
        .and_then(|val| val.as_u64())
        .map(|id| match id {
            0 => TaskId::from("36w8251"), // None
            1 => TaskId::from("36pnwzu"), // v0
            2 => TaskId::from("36w74wp"), // v1
            3 => TaskId::from("36w826q"), // v2
            4 => TaskId::from("36w8281"), // v3
            _ => TaskId::from("36w8251"), // None
        })
}

async fn make_task_subtask_of_milestone_task_if_needed(
    token: &ClickupToken,
    task: &Task,
) -> reqwest::Result<()> {
    if !task_is_in_milestone_space(task) {
        return Ok(());
    }

    if task_is_transitive_subtask_of_milestone_task(token, task).await? {
        return Ok(()); // already good, but note that we do not handle milestone changes correctly yet.
    }

    // The originial domain list, before it was moved to the milestone list
    let domain_list_id = task.list.id.clone();

    // TODO: Make easily configurable

    let destination_task = milestone_destionation_for_task(task)
        .expect("ERROR: Invalid milestone task configuration in binary");

    set_task_parent(token, &task.id, &destination_task).await?; //v0 milestone

    add_task_to_list(token, &task.id, &domain_list_id).await?;

    Ok(())
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

    #[tokio::test]
    #[traced_test]
    async fn task_that_is_in_milestone_list() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w79af")) // Task that was originally in picasso
            .await
            .unwrap();

        assert!(task_is_in_milestone_list(&task));
    }

    #[tokio::test]
    #[traced_test]
    async fn task_that_is_not_in_milestone_list() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w7hq2")) // Test task that is not in milestone list
            .await
            .unwrap();

        assert!(!task_is_in_milestone_list(&task));
    }

    #[tokio::test]
    #[traced_test]
    async fn task_in_different_space_is_not_in_milestone_list() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w78wt")) // Task that is in the unmanaged space
            .await
            .unwrap();

        assert!(!task_is_in_milestone_list(&task));
    }

    #[tokio::test]
    #[traced_test]
    async fn task_that_is_transitive_subtask_of_milestone_task() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w79af")) // Task that is in the unmanaged space
            .await
            .unwrap();

        let is_subtask = task_is_transitive_subtask_of_milestone_task(&CLICKUP_TOKEN, &task)
            .await
            .unwrap();

        assert!(is_subtask);
    }

    #[tokio::test]
    #[traced_test]
    async fn subtask_that_is_transitive_subtask_of_milestone_task() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w7rgq")) // Task that is in the unmanaged space
            .await
            .unwrap();

        let is_subtask = task_is_transitive_subtask_of_milestone_task(&CLICKUP_TOKEN, &task)
            .await
            .unwrap();

        assert!(is_subtask);
    }

    #[tokio::test]
    #[traced_test]
    async fn subsubtask_that_is_transitive_subtask_of_milestone_task() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w7t30")) // Task that is in the unmanaged space
            .await
            .unwrap();

        let is_subtask = task_is_transitive_subtask_of_milestone_task(&CLICKUP_TOKEN, &task)
            .await
            .unwrap();

        assert!(is_subtask);
    }

    #[tokio::test]
    #[traced_test]
    async fn subtask_that_is_not_transitive_subtask_of_milestone_task() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w7q5g")) // Subtask that is not a subtask of a milestone task
            .await
            .unwrap();

        let is_subtask = task_is_transitive_subtask_of_milestone_task(&CLICKUP_TOKEN, &task)
            .await
            .unwrap();

        assert!(!is_subtask);
    }

    #[tokio::test]
    #[traced_test]
    async fn subsubtask_that_is_not_transitive_subtask_of_milestone_task() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w7qpy")) // subsubtask that is not a subtask of a milestone task
            .await
            .unwrap();

        let is_subtask = task_is_transitive_subtask_of_milestone_task(&CLICKUP_TOKEN, &task)
            .await
            .unwrap();

        assert!(!is_subtask);
    }

    #[tokio::test]
    #[traced_test]
    async fn task_should_move_if_needed() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w7wbr")) // task that should get moved
            .await
            .unwrap();

        make_task_subtask_of_milestone_task_if_needed(&CLICKUP_TOKEN, &task)
            .await
            .unwrap();

        let moved_task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w7wbr"))
            .await
            .unwrap();

        assert!(
            task_is_transitive_subtask_of_milestone_task(&CLICKUP_TOKEN, &moved_task)
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn task_that_should_move_to_v2() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w83z6")) // task that should move to v2
            .await
            .unwrap();

        make_task_subtask_of_milestone_task_if_needed(&CLICKUP_TOKEN, &task)
            .await
            .unwrap();

        let moved_task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w83z6"))
            .await
            .unwrap();

        assert!(
            task_is_transitive_subtask_of_milestone_task(&CLICKUP_TOKEN, &moved_task)
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn task_that_should_move_to_v3() {
        let task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w861w")) // task that should move to v2
            .await
            .unwrap();

        make_task_subtask_of_milestone_task_if_needed(&CLICKUP_TOKEN, &task)
            .await
            .unwrap();

        let moved_task = get_task(&CLICKUP_TOKEN, &TaskId::from("36w861w"))
            .await
            .unwrap();

        assert!(
            task_is_transitive_subtask_of_milestone_task(&CLICKUP_TOKEN, &moved_task)
                .await
                .unwrap()
        );
    }
}
