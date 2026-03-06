use crate::{app_state::AppState, message::Message};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message as WebSocketMessage, WebSocket},
    },
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use futures::{SinkExt, StreamExt, channel::mpsc};

pub async fn ws_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Response {
    let user_id = headers
        .get("user_id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u32>().ok());
    let Some(id) = user_id else {
        return StatusCode::NOT_FOUND.into_response();
    };

    ws.on_upgrade(move |ws| self::handle_upgrade(ws, state, id))
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
async fn handle_upgrade(ws: WebSocket, app_state: AppState, user_id: u32) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = mpsc::unbounded::<Message>();

    app_state.map.insert(user_id, tx);
    println!("user id {} connected", user_id);
    let keys: Vec<u32> = app_state.map.iter().map(|e| *e.key()).collect();
    println!("current connection: {:#?}", keys);

    // Sender Task:
    // ws_receiver received the message from sender
    // finds receiver's tx to send message to receiver
    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                WebSocketMessage::Text(text) => {
                    let message: Message = serde_json::from_str(text.as_str())
                        .expect("receiving text is not in valid Message json format");
                    let receiver_id = message.receiver_id;
                    let Some(receiver_tx) = app_state.map.get(&receiver_id) else {
                        println!("did not find user with user id: {}", receiver_id);
                        break;
                    };
                    receiver_tx
                        .unbounded_send(message)
                        .expect("failed to send message to user id: {}");
                }
                WebSocketMessage::Binary(_binary) => {
                    println!("received binary message")
                }
                WebSocketMessage::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    // Receiver Task:
    // rx to listen for messages
    // ws_sender to send the message to receiver
    // store the tx in the map
    tokio::spawn(async move {
        while let Some(message) = rx.next().await {
            ws_sender
                .send(WebSocketMessage::Text(message.into()))
                .await
                .expect("unable to send message to the websocket receipient");
        }
    });
}
