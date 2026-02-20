use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
}

pub struct UserWithPassword {
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    pub secret_hash: Vec<u8>,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionWithToken {
    pub id: String,
    pub secret_hash: Vec<u8>,
    pub created_at: i64,
    pub expires_at: i64,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionValidationResult {
    pub session: Option<SessionWithToken>,
    pub user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
  pub id: i32,
  pub title: String,
  pub content: String,
  pub rating: i32,
  pub user: String,
  pub location: String
}
