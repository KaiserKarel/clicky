use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Hash, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(transparent)]
pub struct ListId(pub(crate) String);

impl From<&str> for ListId {
    fn from(id: &str) -> Self {
        Self(id.to_owned())
    }
}
