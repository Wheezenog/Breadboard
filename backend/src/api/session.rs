use crate::{auth::session::validate_session_token, types::SessionValidationResult};
use aws_sdk_dynamodb::Client;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn validate_session(
    State(client): State<Arc<Client>>,
    Json(token): Json<String>,
) -> impl IntoResponse {
    match validate_session_token(&client, &token).await {
        Some(result) => {
            (StatusCode::OK, Json(result))
        }
        None => {
            println!("Session validation failed");
            (StatusCode::UNAUTHORIZED, SessionValidationResult {
                session: None,
                user: None,
            }.into())
        }
    }
}
