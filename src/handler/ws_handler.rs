use axum::{
    extract::{WebSocketUpgrade, ws::WebSocket},
    response::Response,
};

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    todo!(
        "think how to handle ws connection, should it be separate with message receiving? or should it just store the connection"
    );
}
