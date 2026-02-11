use crate::types::{Session, SessionWithToken};
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, time::SystemTime};

/// Generates a secure random string of 24 characters using the specified character set.
pub fn generate_secure_random_string() -> String {
    const CHARSET: &[u8] = b"abcdefghijkmnpqrstuvwxyz23456789";

    let mut bytes = [0u8; 24];
    rand::fill(&mut bytes[..]);

    let mut id = String::new();
    for i in 0..bytes.len() {
        let index = (bytes[i] >> 3) as usize % CHARSET.len();
        id.push(CHARSET[index] as char);
    }
    id
}

/// Creates a new session
/// Returns the session with the token included
pub async fn create_session(client: &Client, user_id: &str) -> Option<SessionWithToken> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let id = generate_secure_random_string();
    let secret = generate_secure_random_string();
    let secret_hash = hash_secret(&secret);

    let token = format!("{}.{}", id, secret);

    let session: SessionWithToken = SessionWithToken {
        id,
        secret_hash,
        created_at: now as i64,
        token,
    };

    // Add / replace session object under the user table with the user that has the same id

    let session_map = HashMap::from([
        ("id".to_string(), AttributeValue::S(session.id.clone())),
        (
            "secret_hash".to_string(),
            AttributeValue::S(
                session
                    .secret_hash
                    .clone()
                    .iter()
                    .map(|n| n.to_string())
                    .collect(),
            ),
        ),
        (
            "created_at".to_string(),
            AttributeValue::N(session.created_at.to_string()),
        ),
        (
            "token".to_string(),
            AttributeValue::S(session.token.clone()),
        ),
    ]);
    let session_av = AttributeValue::M(session_map);

    let key = HashMap::from([("uuid".to_string(), AttributeValue::S(user_id.to_string()))]);

    let _ = client
        .update_item()
        .table_name("users")
        .set_key(Some(key))
        .update_expression("SET session = :session")
        .expression_attribute_values(":session", session_av)
        .send()
        .await;

    Some(session)
}

/// Hashes the session secret using a secure hashing algorithm
pub fn hash_secret(secret: &str) -> Vec<u8> {
    let secret_bytes = secret.as_bytes();
    let secret_hash_buffer = Sha256::digest(secret_bytes);
    secret_hash_buffer.to_vec()
}

pub async fn validate_session_token(client: &Client, token: &str) -> Option<Session> {
    let token_parts: Vec<&str> = token.split('.').collect();
    if token_parts.len() != 2 {
        return None;
    }

    let session_id = token_parts[0];
    let session_secret = token_parts[1];

    let session = get_session(client, session_id).await;

    if let Some(session) = session {
        let token_secret_hash = hash_secret(session_secret);
        let valid_secret = constant_time_eq(&token_secret_hash, &session.secret_hash);

        if valid_secret {
            return Some(session);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

/// Retrieves a session from the database by its ID
pub async fn get_session(client: &Client, session_id: &str) -> Option<Session> {
    // Query the database for the session with the given ID
    // the session is a map with key "session" held inside the user table
    // we need to look through the users table to find a user that has a "sessions" item, and that item has an "id" value that matches session_id
    let result = client
        .scan()
        .table_name("users")
        .filter_expression("contains(sessions, :session_id)")
        .expression_attribute_values(":session_id", AttributeValue::S(session_id.to_string()))
        .send()
        .await;

    if let Ok(output) = result {
        // Not errored
        if let Some(items) = output.items {
            // Items exist
            for item in items {
                if let Some(session_av) = item.get("session") {
                    if let AttributeValue::M(session_map) = session_av {
                        if let Some(AttributeValue::S(id)) = session_map.get("id") {
                            if id == session_id {
                                // Found the session
                                let secret_hash = if let Some(AttributeValue::S(secret_hash_str)) =
                                    session_map.get("secret_hash")
                                {
                                    // Split returns a proper iterator of &str, so we can filter_map and collect into a Vec<u8>
                                    // The has will not contain a comma, so the split will return an iterator with one element, which is the original string
                                    secret_hash_str
                                        .split(',')
                                        .filter_map(|s| s.parse::<u8>().ok())
                                        .collect()
                                } else {
                                    return None;
                                };
                                let created_at = if let Some(AttributeValue::N(created_at_str)) =
                                    session_map.get("created_at")
                                {
                                    created_at_str.parse::<i64>().unwrap_or(0)
                                } else {
                                    0
                                };
                                return Some(Session {
                                    id: id.clone(),
                                    secret_hash,
                                    created_at,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub async fn delete_session(client: &Client, session_id: &str) -> bool {
    // Similar to get_session, we need to find the user that has the session with the given ID, and then remove that session from the user's sessions
    let result = client
        .scan()
        .table_name("users")
        .filter_expression("contains(sessions, :session_id)")
        .expression_attribute_values(":session_id", AttributeValue::S(session_id.to_string()))
        .send()
        .await;

    if let Ok(output) = result {
        if let Some(items) = output.items {
            for item in items {
                if let Some(session_av) = item.get("session") {
                    if let AttributeValue::M(session_map) = session_av {
                        if let Some(AttributeValue::S(id)) = session_map.get("id") {
                            if id == session_id {
                                // Found the session, now we need to remove it from the user's sessions
                                let key = HashMap::from([(
                                    "uuid".to_string(),
                                    item.get("uuid").unwrap().clone(),
                                )]);
                                let _ = client
                                    .update_item()
                                    .table_name("users")
                                    .set_key(Some(key))
                                    .update_expression("REMOVE session")
                                    .send()
                                    .await;
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Constant time comparison of two byte slices to prevent timing attacks
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result: u8 = 0;
    for i in 0..a.len() {
        result |= a[i] ^ b[i];
    }

    return result == 0;
}
