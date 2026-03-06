use crate::{app_state::AppState, message::Message};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message as WebSocketMessage, WebSocket},
    },
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use futures::{
    SinkExt, StreamExt,
    channel::mpsc::{self, UnboundedReceiver},
    stream::{SplitSink, SplitStream},
};

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

/// split the websocket into sending half and receiving half
/// create a mpsc channel to handle the send and the receive concurrently
async fn handle_upgrade(ws: WebSocket, app_state: AppState, user_id: u32) {
    let (ws_sender, ws_receiver) = ws.split();
    let (tx, rx) = mpsc::unbounded::<Message>();

    app_state.map.insert(user_id, tx);
    println!("user id {} connected", user_id);
    let keys: Vec<u32> = app_state.map.iter().map(|e| *e.key()).collect();
    println!("current connection: {:#?}", keys);

    let send_handle = tokio::spawn(handle_msg_send(ws_receiver, app_state.clone()));

    let receive_handle = tokio::spawn(handle_msg_receive(rx, ws_sender));

    tokio::select! {
        _ = send_handle => {println!("sender task ended!")},
        _ = receive_handle => {println!("receiver task ended!")},
    }

    app_state.map.remove(&user_id);
}

/// Sender Task
/// ws_receiver received the message from sender
/// finds receiver's tx to send message to receiver
async fn handle_msg_send(mut ws_receiver: SplitStream<WebSocket>, app_state: AppState) {
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
}

/// Receiver Task:
/// rx to listen for messages
/// ws_sender to send the message to receiver
/// store the tx in the map
async fn handle_msg_receive(
    mut rx: UnboundedReceiver<Message>,
    mut ws_sender: SplitSink<WebSocket, WebSocketMessage>,
) {
    while let Some(message) = rx.next().await {
        ws_sender
            .send(WebSocketMessage::Text(message.into()))
            .await
            .expect("unable to send message to the websocket receipient");
    }
}
