use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, http::StatusCode};
use crate::auth::session::validate_session_token;
use aws_sdk_dynamodb::Client;

async fn register_user(
    State(client): State<Arc<Client>>,
    username: String,
    password: String,
) -> impl IntoResponse {
}
