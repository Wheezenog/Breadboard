use serde::Serialize;

pub struct User {
    pub username: String,
}

pub struct UserWithPassword {
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize)]
pub struct Session {
    pub id: String,
    pub secret_hash: Vec<u8>,
    pub created_at: i64,
    pub expires_at: i64,
}

pub struct SessionWithToken {
    pub id: String,
    pub secret_hash: Vec<u8>,
    pub created_at: i64,
    pub expires_at: i64,
    pub token: String,
}

#[derive(Serialize)]
pub struct SessionValidationResult {
    pub session: Option<Session>,
    pub user: Option<String>,
}
