#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, Hash)]
#[serde(transparent)]
pub struct ListId(pub(crate) u128);

impl From<u128> for ListId {
    fn from(n: u128) -> Self {
        Self(n)
    }
}
