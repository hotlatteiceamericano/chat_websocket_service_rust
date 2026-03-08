use anyhow::Result;
use axum::{body::Bytes, extract::ws::Utf8Bytes};
use futures::channel::mpsc::TrySendError;
use thiserror::Error;

use crate::{app_state::AppState, message::Message};

pub fn handle_text(text: &Utf8Bytes, app_state: &AppState) -> Result<(), MessageHandleError> {
    let message: Message = serde_json::from_str(text.as_str())?;

    let receiver_tx =
        app_state
            .map
            .get(&message.receiver_id)
            .ok_or(MessageHandleError::ReceiverNotFound {
                id: message.receiver_id,
            })?;

    receiver_tx.unbounded_send(message)?;

    Ok(())
}

pub fn handle_binary(_binary: &Bytes, _app_state: &AppState) -> Result<(), MessageHandleError> {
    Ok(())
}

/// Custom error to conclude three different types of error
/// from distinct crates into this enum
/// It uses thiserror's derive helpers to implement
/// Error trait (for anyhow)
/// and From<T> trait (convert to MessageHandleError)
#[derive(Debug, Error)]
pub enum MessageHandleError {
    #[error("user id not found: {id}")]
    ReceiverNotFound { id: u32 },

    #[error("invalid message format")]
    InvalidMessageFormat {
        #[from]
        error: serde_json::Error,
    },

    #[error("error when sending messages to receiver's transimitter")]
    MPSC(#[from] TrySendError<Message>),
}
