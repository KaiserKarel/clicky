use bytes::Bytes;
use uuid::Uuid;

#[derive(Debug)]
pub struct Event {
    pub id: Uuid,
    pub payload: Bytes,
    pub received_at: chrono::DateTime<chrono::Utc>,
    pub webhook_id: String,
}

impl Event {
    pub fn new(payload: Bytes, webhook_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            payload,
            webhook_id,
            received_at: chrono::Utc::now(),
        }
    }
}
