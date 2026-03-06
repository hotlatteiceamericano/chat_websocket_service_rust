use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender_id: u32,
    pub receiver_id: u32,
    // todo: update to Vec<u32>
    pub payload: String,
    // msg_type: MessageType
}

// impl From<&str> for Message {
//     fn from(value: &str) -> Self {
//         let json = serde_json::Deserializer::from(value);
//         Self {
//             payload: String::from(value),
//         }
//     }
// }
