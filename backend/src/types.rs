pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub struct Session {
    pub id: String,
    pub secret_hash: Vec<u8>,
    pub created_at: i64,
}

pub struct SessionWithToken {
    pub id: String,
    pub secret_hash: Vec<u8>,
    pub created_at: i64,
    pub token: String,
}