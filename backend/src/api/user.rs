use crate::auth::{
    self,
    password::hash_password,
    session::create_session,
    user::{create_user, get_user_by_username},
    verification::{verify_password_strength, verify_username},
};
use aws_sdk_dynamodb::Client;
use axum::{self, Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct UserRequest {
    username: String,
    password: String,
}


#[axum::debug_handler]
pub async fn register_user(
    State(client): State<Arc<Client>>,
    Json(payload): Json<UserRequest>,
) -> impl IntoResponse {
    let username = payload.username;
    let password = payload.password;

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
            let session = create_session(&client, user.username).await;

            if let Some(session) = session {
                // return the session token so the frontend can set the cookie
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

pub async fn login_user(
    State(client): State<Arc<Client>>,
    Json(payload): Json<UserRequest>,
) -> impl IntoResponse {
    let username = payload.username;
    let password = payload.password;

    let password_hash = hash_password(password);

    let user = get_user_by_username(&client, username).await;

    if let Some(user) = user {
        if let Some(hash) = password_hash {
            if hash == hash {
                let session = create_session(&client, user.username).await;
                if let Some(session) = session {
                    return (StatusCode::OK, session.token.to_string());
                }
            }
        }
    }
    (
        StatusCode::UNAUTHORIZED,
        "Invalid username or password".to_string(),
    )
}

pub async fn logout_user(
    State(client): State<Arc<Client>>,
    Json(token): Json<String>,
) -> impl IntoResponse {
  println!("Logging out user");
    let token_parts: Vec<&str> = token.split('.').collect();
    if token_parts.len() != 2 {
      println!("Token invalid twin");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Logout failed, invalid token"),
        );
    }

    let session_id = token_parts[0];

    
    if auth::session::delete_session(&client, session_id).await {
      println!("Sucesfully logged out");
        (StatusCode::OK, Json("Logout Success"))
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to log out user"),
        )
    }
}
