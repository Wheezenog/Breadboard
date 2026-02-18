use std::sync::Arc;

use crate::{auth::password::hash_password, types::{User, UserWithPassword}};
use aws_sdk_dynamodb::{Client, types::AttributeValue};

pub async fn create_user(client: &Arc<Client>, username: String, password: String) -> Option<User> {
    let password_hash = hash_password(password);

    let username_av = AttributeValue::S(username.to_string());
    let password_hash_av = AttributeValue::S(password_hash.unwrap_or_default());

    let _ = client
        .put_item()
        .table_name("Users")
        .item("username", username_av)
        .item("password_hash", password_hash_av)
        .send()
        .await;

    return Some(User {
        username: username.to_string(),
    });
}

pub async fn get_user_by_username(client: &Arc<Client>, username: String) -> Option<UserWithPassword> {
    let request = client
        .scan()
        .table_name("Users")
        .filter_expression("username = :username")
        .expression_attribute_values(":username", AttributeValue::S(username.to_string()))
        .send()
        .await;

    match request {
        Ok(output) => {
            if let Some(items) = output.items {
                if let Some(item) = items.first() {
                    let username = item.get("username")?.as_s().ok();
                    if let Some(username) = username {
                        return Some(UserWithPassword {
                            username: username.to_string(),
                            password_hash: get_user_password_hash(client, username).await?,
                        });
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}

pub async fn get_user_password_hash(client: &Client, username: &str) -> Option<String> {
    let request = client
        .scan()
        .table_name("Users")
        .filter_expression("username = :username")
        .expression_attribute_values(":username", AttributeValue::S(username.to_string()))
        .send()
        .await;

    match request {
        Ok(output) => {
            if let Some(items) = output.items {
                if let Some(item) = items.first() {
                    let password_hash = item.get("password_hash")?.as_s().ok();
                    if let Some(password_hash) = password_hash {
                        return Some(password_hash.to_string());
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}
