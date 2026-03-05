use crate::app_state::AppState;
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message as WebSocketMessage, WebSocket},
    },
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use futures::{StreamExt, channel::mpsc};

pub async fn ws_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Response {
    let user_id = headers.get("user_id").and_then(|v| v.to_str().ok());
    let Some(id) = user_id else {
        return StatusCode::NOT_FOUND.into_response();
    };

    ws.on_upgrade(move |ws| self::handle_upgrade(ws, state))
}

// create connection and store it with id
// first task:
//   ws_receiver receives messages
//   find receiver's tx and send to it
//   store message in db
// second task:
//   receiving messages from the rx
//   send messages to ws_sender
// StatusCode::OK.into_response()
async fn handle_upgrade(mut ws: WebSocket, app_state: AppState) {
    let (ws_sender, mut ws_receiver) = ws.split();
    let (tx, rx) = mpsc::unbounded::<String>();

    // sending messages out
    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            WebSocketMessage::Text(text) => {
                // let message = Message::from(text.to_string()); let receiver_id = message.get_receiver_id();
                // let receiver_tx = app_state.map.get(receiver_id);
                println!("received text message: {}", text.to_string());
            }
            WebSocketMessage::Binary(binary) => {
                println!("received binary message")
            }
            WebSocketMessage::Close(_) => {
                break;
            }
            _ => {}
        }
    }

    // receiving messages
    // store the tx in the map
}
