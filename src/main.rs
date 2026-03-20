use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state, routing::get};
use chat_websocket_service_rust::{
    app_state::AppState, auth_middleware::auth, handler::ws_handler,
};
use dashmap::DashMap;

#[tokio::main]
async fn main() {
    // eastablish a axum websocket server
    // subscribe to redis pub/sub for messages from other
    // websocket handler should be looking up the connection using user id
    // keep in mind that the connection lookup will be start with in-server
    // and later become cross-server using Redis Pub/Sub
    let state = AppState::new(Arc::new(DashMap::new()));
    let app = Router::new()
        .route("/ws", get(ws_handler::ws_handler))
        // .layer(from_fn_with_state(state.clone(), auth))
        .with_state(state);
    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("websocket server starts");
    axum::serve(listner, app).await.unwrap();
}
