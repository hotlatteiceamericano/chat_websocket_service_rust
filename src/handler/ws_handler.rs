use crate::{
    app_state::AppState,
    handler::msg_handler::{self},
};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message as WebSocketMessage, WebSocket},
    },
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use chat_common::{message::Message, message_handle_error::MessageHandleError};
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
        .get("User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u32>().ok());
    let Some(id) = user_id else {
        return StatusCode::BAD_REQUEST.into_response();
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
        println!("received messages from client");
        let result = match msg {
            WebSocketMessage::Text(text) => msg_handler::handle_text(&text, &app_state),
            WebSocketMessage::Binary(binary) => msg_handler::handle_binary(&binary, &app_state),
            _ => {
                println!("websocket received message with unsupported type, dropping");
                continue;
            }
        };

        if let Err(e) = result {
            match e {
                MessageHandleError::ReceiverNotFound { id } => {
                    println!("receiver id: {} not found, dropping the message", id);
                }
                MessageHandleError::InvalidMessageFormat { error } => {
                    println!("received message is in invalid format, err: {:?}", error);
                }
                _ => println!("unsupported error type when sending message to websocket receiver."),
            }
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
        let text = serde_json::to_string(&message).unwrap();
        if let Err(e) = ws_sender.send(WebSocketMessage::Text(text.into())).await {
            println!("unable to send message to the websocket recipient: {}", e);
            break;
        }
    }
}
