use std::sync::Arc;

use dashmap::DashMap;
use futures::channel::mpsc::UnboundedSender;

use crate::message::Message;

#[derive(Clone)]
pub struct AppState {
    pub map: Arc<DashMap<u32, UnboundedSender<Message>>>,
}

impl AppState {
    pub fn new(map: Arc<DashMap<u32, UnboundedSender<Message>>>) -> Self {
        Self { map }
    }
}
