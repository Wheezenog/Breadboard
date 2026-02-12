use crate::{
    auth::{
        session::create_session,
        user::create_user,
        verification::{verify_password_strength, verify_username},
    },
};
use aws_sdk_dynamodb::Client;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

async fn register_user(
    State(client): State<Arc<Client>>,
    username: String,
    password: String,
) -> impl IntoResponse {
    // Verify username availability
    if verify_username(&client, &username).await {
        return (
            StatusCode::BAD_REQUEST,
            "Username is already taken".to_string(),
        );
    }

    // Verify password strength
    if verify_password_strength(&password).await {
        return (StatusCode::BAD_REQUEST, "Weak password".to_string());
    }

    let user = create_user(&client, username, password).await;

    match user {
        Some(user) => {
            let session = create_session(&client, &user.username).await;

            if let Some(session) = session {
                // Set session cookie or return session token as needed
                // For example, you could return the session token in the response body
                return (StatusCode::OK, session.token.to_string());
            } else {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create session".to_string(),
                );
            }
        }
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create user".to_string(),
            );
        }
    }
}
