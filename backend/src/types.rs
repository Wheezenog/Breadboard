pub struct User {
    pub username: String,
}

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