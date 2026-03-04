use axum::{Router, routing::get};
use chat_websocket_service_rust::handler::ws_handler;

#[tokio::main]
async fn main() {
    // eastablish a axum websocket server
    // subscribe to redis pub/sub for messages from other
    // websocket handler should be looking up the connection using user id
    // keep in mind that the connection lookup will be start with in-server
    // and later become cross-server using Redis Pub/Sub
    let app = Router::new().route("/ws", get(ws_handler::ws_handler));
    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
