use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state, routing::get};
use chat_websocket_service_rust::{
    app_state::AppState, auth_middleware::auth, handler::ws_handler,
};
use dashmap::DashMap;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // eastablish a axum websocket server
    // subscribe to redis pub/sub for messages from other
    // websocket handler should be looking up the connection using user id
    // keep in mind that the connection lookup will be start with in-server
    // and later become cross-server using Redis Pub/Sub
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let jwt_secret = std::env::var("JWT_SECRET").expect("jwt secret required");
    let state = AppState::new(Arc::new(DashMap::new()), jwt_secret);
    let app = Router::new()
        .route("/ws", get(ws_handler::ws_handler))
        .layer(from_fn_with_state(state.clone(), auth))
        .with_state(state);
    let listner = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    tracing::info!("websocket server starts");
    axum::serve(listner, app).await.unwrap();
}
