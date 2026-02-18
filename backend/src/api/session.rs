use crate::auth::session::validate_session_token;
use aws_sdk_dynamodb::Client;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

pub async fn validate_session(
    State(client): State<Arc<Client>>,
    Json(token): Json<String>,
) -> impl IntoResponse {
    match validate_session_token(&client, &token).await {
        Some(result) => (StatusCode::OK, serde_json::to_string(&result).unwrap()),
        None => (
            StatusCode::UNAUTHORIZED,
            "Invalid session token".to_string(),
        ),
    }
}
