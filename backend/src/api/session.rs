use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, http::StatusCode};
use crate::auth::session::validate_session_token;
use aws_sdk_dynamodb::Client;

pub async fn validate_session(State(client): State<Arc<Client>>, token: String) -> impl IntoResponse {
    match validate_session_token(&client, &token).await {
        Some(result) => (StatusCode::OK, serde_json::to_string(&result).unwrap()),
        None => (
            StatusCode::UNAUTHORIZED,
            "Invalid session token".to_string(),
        ),
    }
}
