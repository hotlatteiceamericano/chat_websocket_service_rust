pub struct Message {
    // sender_id: u32,
    // receiver_id: u32,
    // todo: update to Vec<u32>
    payload: String,
    // msg_type: MessageType
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Self { payload: value }
    }
}
