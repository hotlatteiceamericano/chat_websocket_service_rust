use anyhow::Result;
use axum::{body::Bytes, extract::ws::Utf8Bytes};
use chat_common::{message::Message, message_handle_error::MessageHandleError};

use crate::app_state::AppState;

pub fn handle_text(text: &Utf8Bytes, app_state: &AppState) -> Result<(), MessageHandleError> {
    let message: Message = Message::try_from(text)?;

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

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use axum::extract::ws::Utf8Bytes;
    use chat_common::message::Message;
    use dashmap::DashMap;
    use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
    use rstest::{fixture, rstest};

    use crate::{
        app_state::AppState,
        handler::msg_handler::{self, MessageHandleError},
    };

    // receiver needs to be returned so that it can live until the test run
    #[fixture]
    fn app_state() -> (AppState, UnboundedReceiver<Message>) {
        let map: DashMap<u32, UnboundedSender<Message>> = DashMap::new();
        let (tx, rx) = mpsc::unbounded::<Message>();
        map.insert(1, tx);
        (AppState::new(Arc::new(map)), rx)
    }

    #[fixture]
    fn valid_msg() -> Utf8Bytes {
        let msg = Message {
            sender_id: 0,
            receiver_id: 1,
            payload: String::from(""),
        };
        Utf8Bytes::from(
            serde_json::to_string(&msg).expect("was not able to serialize Message to JSON"),
        )
    }

    #[fixture]
    fn unexist_receiver() -> Utf8Bytes {
        let msg = Message {
            sender_id: 0,
            receiver_id: 999,
            payload: String::from(""),
        };
        Utf8Bytes::from(
            serde_json::to_string(&msg).expect("was not able to serialize Message to JSON"),
        )
    }

    #[fixture]
    fn invalid_msg() -> Utf8Bytes {
        Utf8Bytes::from("invalid message")
    }

    #[rstest]
    fn test_send_suceess(valid_msg: Utf8Bytes, app_state: (AppState, UnboundedReceiver<Message>)) {
        let (app_state, _rx) = app_state;
        let result = msg_handler::handle_text(&valid_msg, &app_state);
        assert!(
            matches!(result, Ok(_)),
            "expect success but found an error: {:?}",
            result.err()
        );
    }

    #[rstest]
    fn test_send_invalid_format_msg(
        invalid_msg: Utf8Bytes,
        app_state: (AppState, UnboundedReceiver<Message>),
    ) {
        let (app_state, _rx) = app_state;
        let result = msg_handler::handle_text(&invalid_msg, &app_state);
        assert!(matches!(
            result,
            Err(MessageHandleError::InvalidMessageFormat { error })
        ),);
    }

    #[rstest]
    fn test_no_receiver(
        unexist_receiver: Utf8Bytes,
        app_state: (AppState, UnboundedReceiver<Message>),
    ) {
        let (app_state, _rx) = app_state;
        let result = msg_handler::handle_text(&unexist_receiver, &app_state);
        assert!(matches!(
            result,
            Err(MessageHandleError::ReceiverNotFound { id: 999 })
        ),);
    }
}
