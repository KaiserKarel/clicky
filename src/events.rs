use bytes::Bytes;
use fasthash::metro;

#[derive(Debug)]
pub struct Event {
    pub id: u64,
    pub payload: Bytes,
    pub received_at: chrono::DateTime<chrono::Utc>,
    pub webhook_id: String,
}

impl Event {
    pub fn new(payload: Bytes, webhook_id: String) -> Self {
        Self {
            id: metro::hash64(&payload),
            payload,
            webhook_id,
            received_at: chrono::Utc::now(),
        }
    }
}
