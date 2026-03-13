use axum::extract::ws::Utf8Bytes;
use serde::{Deserialize, Serialize};

use crate::handler::msg_handler::MessageHandleError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub sender_id: u32,
    pub receiver_id: u32,
    // todo: update to Vec<u32>
    pub payload: String,
    // msg_type: MessageType
}

impl TryFrom<&Utf8Bytes> for Message {
    type Error = MessageHandleError;

    fn try_from(value: &Utf8Bytes) -> Result<Self, Self::Error> {
        serde_json::from_str(value.as_str())
            .map_err(|e| MessageHandleError::InvalidMessageFormat { error: e })
    }
}
