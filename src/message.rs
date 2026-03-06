use axum::extract::ws::Utf8Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender_id: u32,
    pub receiver_id: u32,
    // todo: update to Vec<u32>
    pub payload: String,
    // msg_type: MessageType
}

impl Into<Utf8Bytes> for Message {
    fn into(self) -> Utf8Bytes {
        let json = serde_json::to_string(&self).expect("unable to serialize a Message to json");
        json.into()
    }
}
