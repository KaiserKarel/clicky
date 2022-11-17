#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TeamId(pub(crate) u128);

impl From<u128> for TeamId {
    fn from(n: u128) -> Self {
        Self(n)
    }
}
