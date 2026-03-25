use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::app_state::AppState;

pub async fn auth(State(_state): State<AppState>, request: Request, next: Next) -> Response {
    let Some(_auth) = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
    else {
        tracing::error!("missing Authorization from the request");
        return StatusCode::UNAUTHORIZED.into_response();
    };

    next.run(request).await
}
