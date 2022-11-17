pub mod request {
    use super::events::Event;
    use crate::clickup::team::TeamId;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Clone, Hash)]
    pub struct CreateWebhookParameters {
        pub space_id: Option<u128>,
        pub folder_id: Option<u128>,
        pub list_id: Option<u128>,
        pub task_id: Option<u128>,
        pub endpoint: String,
        pub events: enumset::EnumSet<Event>,
    }

    #[derive(Serialize, Clone)]
    struct CreateWebhookParametersInner {
        pub space_id: Option<u128>,
        pub folder_id: Option<u128>,
        pub list_id: Option<u128>,
        pub task_id: Option<u128>,
        pub endpoint: String,
        pub events: Vec<Event>,
    }

    impl From<CreateWebhookParameters> for CreateWebhookParametersInner {
        fn from(p: CreateWebhookParameters) -> Self {
            CreateWebhookParametersInner {
                space_id: p.space_id,
                folder_id: p.folder_id,
                list_id: p.list_id,
                task_id: p.task_id,
                endpoint: p.endpoint,
                events: p.events.into_iter().collect(),
            }
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateWebhookResponse {
        pub id: String,
        pub webhook: Webhook,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Webhook {
        pub id: String,
        pub userid: i64,
        #[serde(rename = "team_id")]
        pub team_id: i64,
        pub endpoint: String,
        #[serde(rename = "client_id")]
        pub client_id: Option<String>,
        pub events: Vec<Event>,
        #[serde(rename = "task_id")]
        pub task_id: Option<String>,
        #[serde(rename = "list_id")]
        pub list_id: Option<String>,
        #[serde(rename = "folder_id")]
        pub folder_id: Option<String>,
        #[serde(rename = "space_id")]
        pub space_id: Option<String>,
        // pub health: Health,
        pub secret: Option<String>,
    }

    // #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    // #[serde(rename_all = "camelCase")]
    // pub struct Health {}

    impl<U: Into<String>, V: Into<enumset::EnumSet<Event>>> From<(U, V)> for CreateWebhookParameters {
        fn from(params: (U, V)) -> Self {
            let (endpoint, events) = (params.0.into(), params.1.into());
            CreateWebhookParameters {
                endpoint,
                events,
                space_id: None,
                folder_id: None,
                list_id: None,
                task_id: None,
            }
        }
    }

    pub async fn create_webhook(
        team_id: impl Into<TeamId>,
        params: impl Into<CreateWebhookParameters>,
        authorization: &str,
    ) -> Result<String, reqwest::Error> {
        let params: CreateWebhookParametersInner = params.into().into();
        let client = reqwest::Client::new();

        let url = format!(
            "https://api.clickup.com/api/v2/team/{}/webhook",
            team_id.into().0
        );

        client
            .post(url)
            .header(reqwest::header::AUTHORIZATION, authorization)
            .json(&params)
            .send()
            .await?
            .text()
            .await
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[tokio::test]
        async fn test_create_webhook_works() {
            create_webhook(
                1,
                ("https://yourdomain.com/webhook", Event::all()),
                "foobazle",
            )
            .await
            .unwrap();
        }
    }
}

pub mod events {
    use enumset::{EnumSet, EnumSetType};

    #[derive(EnumSetType, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Event {
        TaskCreated,
        TaskUpdated,
        TaskDeleted,
        TaskPriorityUpdated,
        TaskStatusUpdated,
        TaskAssigneeUpdated,
        TaskDueDateUpdated,
        TaskTagUpdated,
        TaskMoved,
        TaskCommentPosted,
        TaskCommentUpdated,
        TaskTimeEstimateUpdated,
        TaskTimeTrackedUpdated,
        ListCreated,
        ListUpdated,
        ListDeleted,
        FolderCreated,
        FolderUpdated,
        FolderDeleted,
        SpaceCreated,
        SpaceUpdated,
        SpaceDeleted,
        GoalCreated,
        GoalUpdated,
        GoalDeleted,
        KeyResultCreated,
        KeyResultUpdated,
        KeyResultDeleted,
    }

    impl Event {
        pub fn all() -> EnumSet<Self> {
            EnumSet::all()
        }
    }

    impl ToString for Event {
        fn to_string(&self) -> String {
            self.as_ref().to_string()
        }
    }

    impl AsRef<str> for Event {
        fn as_ref(&self) -> &str {
            match self {
                Event::TaskCreated => "taskCreated",
                Event::TaskUpdated => "taskUpdated",
                Event::TaskDeleted => "taskDeleted",
                Event::TaskPriorityUpdated => "taskPriorityUpdated",
                Event::TaskStatusUpdated => "taskStatusUpdated",
                Event::TaskAssigneeUpdated => "taskAssigneeUpdated",
                Event::TaskDueDateUpdated => "taskDueDateUpdated",
                Event::TaskTagUpdated => "taskTagUpdated",
                Event::TaskMoved => "taskMoved",
                Event::TaskCommentPosted => "taskCommentPosted",
                Event::TaskCommentUpdated => "taskCommentUpdated",
                Event::TaskTimeEstimateUpdated => "taskTimeEstimateUpdated",
                Event::TaskTimeTrackedUpdated => "taskTimeTrackedUpdated",
                Event::ListCreated => "listCreated",
                Event::ListUpdated => "listUpdated",
                Event::ListDeleted => "listDeleted",
                Event::FolderCreated => "folderCreated",
                Event::FolderUpdated => "folderUpdated",
                Event::FolderDeleted => "folderDeleted",
                Event::SpaceCreated => "spaceCreated",
                Event::SpaceUpdated => "spaceUpdated",
                Event::SpaceDeleted => "spaceDeleted",
                Event::GoalCreated => "goalCreated",
                Event::GoalUpdated => "goalUpdated",
                Event::GoalDeleted => "goalDeleted",
                Event::KeyResultCreated => "keyResultCreated",
                Event::KeyResultUpdated => "keyResultUpdated",
                Event::KeyResultDeleted => "keyResultDeleted",
            }
        }
    }
}
