use crate::types::{Session, SessionValidationResult, SessionWithToken, User};
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, sync::Arc, time::SystemTime};

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
pub async fn create_session(client: &Arc<Client>, username: String) -> Option<SessionWithToken> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let id = generate_secure_random_string();
    let secret = generate_secure_random_string();
    let secret_hash = hash_secret(&secret);
    let expires_at = now + 60 * 60 * 24 * 7;

    let token = format!("{}.{}", id, secret);

    let session: SessionWithToken = SessionWithToken {
        id: id.clone(),
        secret_hash: secret_hash.clone(),
        created_at: now as i64,
        expires_at: expires_at as i64,
        token: token.clone(),
    };

    let session_map = HashMap::from([
        ("id".to_string(), AttributeValue::S(id)),
        (
            "secret_hash".to_string(),
            AttributeValue::S(
                secret_hash
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ),
        (
            "created_at".to_string(),
            AttributeValue::N(session.created_at.to_string()),
        ),
        (
            "expires_at".to_string(),
            AttributeValue::N(session.expires_at.to_string()),
        ),
    ]);

    let session_av = AttributeValue::M(session_map);
    let key = HashMap::from([("username".to_string(), AttributeValue::S(username.clone()))]);

    let response = client
        .update_item()
        .table_name("Users")
        .set_key(Some(key))
        .update_expression("SET #session = :session")
        .expression_attribute_names("#session", "session")
        .expression_attribute_values(":session", session_av)
        .send()
        .await;

    match response {
        Ok(_) => Some(session),
        Err(_) => None,
    }
}

/// Hashes the session secret using a secure hashing algorithm
pub fn hash_secret(secret: &str) -> Vec<u8> {
    let secret_bytes = secret.as_bytes();
    let secret_hash_buffer = Sha256::digest(secret_bytes);
    secret_hash_buffer.to_vec()
}

pub async fn validate_session_token(
    client: &Client,
    token: &str,
) -> Option<SessionValidationResult> {
    let token_parts: Vec<&str> = token.split('.').collect();
    if token_parts.len() != 2 {
        return None;
    }

    let session_id = token_parts[0];
    let session_secret = token_parts[1];

    let result = get_session(client, session_id).await;

    if let Some((user, session)) = result {
        let token_secret_hash = hash_secret(session_secret);
        let valid_secret = constant_time_eq(&token_secret_hash, &session.secret_hash);

        if valid_secret {
            let session_with_token = SessionWithToken {
                id: session.id,
                secret_hash: session.secret_hash,
                created_at: session.created_at,
                expires_at: session.expires_at,
                token: token.to_string(),
            };

            return Some(SessionValidationResult {
                session: Some(session_with_token),
                user: Some(user),
            });
        } else {
            return None;
        }
    } else {
        return None;
    }
}

/// Retrieves a session from the database by its ID
pub async fn get_session(client: &Client, session_id: &str) -> Option<(User, Session)> {
    // Query the database for the session with the given ID
    // the session is a map with key "session" held inside the user table
    // we need to look through the users table to find a user that has a "sessions" item, and that item has an "id" value that matches session_id'

    let filter_expression = "#session.#id = :session_id".to_string();

    let result = client
        .scan()
        .table_name("Users")
        .filter_expression(filter_expression)
        .expression_attribute_names("#session", "session")
        .expression_attribute_names("#id", "id")
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
                                // Found the session, now we need to return it as a Session struct
                                let secret_hash_str = session_map
                                    .get("secret_hash")
                                    .and_then(|av| av.as_s().ok())?;
                                let secret_hash = secret_hash_str
                                    .split(',')
                                    .filter_map(|s| s.parse::<u8>().ok())
                                    .collect::<Vec<u8>>();

                                let created_at = session_map
                                    .get("created_at")
                                    .and_then(|av| av.as_n().ok())?
                                    .parse::<i64>()
                                    .ok()?;

                                let expires_at = session_map
                                    .get("expires_at")
                                    .and_then(|av| av.as_n().ok())?
                                    .parse::<i64>()
                                    .ok()?;

                                let username =
                                    item.get("username").and_then(|av| av.as_s().ok())?;

                                return Some((
                                    User {
                                        username: username.clone(),
                                    },
                                    Session {
                                        id: id.clone(),
                                        secret_hash,
                                        created_at,
                                        expires_at,
                                    },
                                ));
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
    let result = get_session(client, session_id).await;

    if let Some((user, session)) = result {
        let session_map = HashMap::from([
            ("id".to_string(), AttributeValue::S(session.id)),
            (
                "secret_hash".to_string(),
                AttributeValue::S(
                    session
                        .secret_hash
                        .iter()
                        .map(|b| b.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            ),
            (
                "created_at".to_string(),
                AttributeValue::N(session.created_at.to_string()),
            ),
            (
                "expires_at".to_string(),
                AttributeValue::N(session.expires_at.to_string()),
            ),
        ]);

        let session_av = AttributeValue::M(session_map);

        match client
            .delete_item()
            .table_name("Users")
            .key(user.username, session_av)
            .send()
            .await
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    } else {
        return false;
    }
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
