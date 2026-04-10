use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use chat_common::claim::Claims;
use jsonwebtoken::{DecodingKey, Validation};

use crate::app_state::AppState;

pub async fn auth(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let Some(auth_value) = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    else {
        tracing::error!("missing Authorization from the request, rejecting");
        return (StatusCode::UNAUTHORIZED, "missing authorization").into_response();
    };

    let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    match jsonwebtoken::decode::<Claims>(auth_value, &decoding_key, &validation) {
        Ok(token_data) => {
            let mut request = request;
            request.extensions_mut().insert(token_data.claims.sub);
            next.run(request).await
        }
        Err(e) => {
            tracing::error!("Authentication failed: {:?}", e);
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}
