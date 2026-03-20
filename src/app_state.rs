use std::sync::Arc;

use chat_common::message::Message;
use dashmap::DashMap;
use futures::channel::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct AppState {
    pub map: Arc<DashMap<u32, UnboundedSender<Message>>>,
}

impl AppState {
    pub fn new(map: Arc<DashMap<u32, UnboundedSender<Message>>>) -> Self {
        Self { map }
    }
}
