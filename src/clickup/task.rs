use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Hash, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(transparent)]
pub struct TaskId(pub(crate) String);

impl From<&str> for TaskId {
    fn from(id: &str) -> Self {
        Self(id.to_owned())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub parent: Option<String>,
    pub custom_fields: Vec<CustomField>,
    pub list: List,
    pub folder: Folder,
    pub space: Space,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomField {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub value: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldValue {
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct List {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Space {
    pub id: String,
}
