use std::{collections::HashMap, sync::Arc};

use futures::{channel::mpsc::UnboundedSender, lock::Mutex};

#[derive(Clone)]
pub struct AppState {
    pub map: Arc<Mutex<HashMap<u32, UnboundedSender<String>>>>,
}

impl AppState {
    pub fn new(map: Arc<Mutex<HashMap<u32, UnboundedSender<String>>>>) -> Self {
        Self { map }
    }
}
