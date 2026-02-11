use crate::{
    auth::password::{hash_password, verify_password},
    types::User,
};
use aws_sdk_dynamodb::{Client, types::AttributeValue};

pub async fn create_user(client: &Client, username: &str, password: &str) -> Option<User> {
    let password_hash = hash_password(password);

    let username_av = AttributeValue::S(username.to_string());
    let password_hash_av = AttributeValue::S(password_hash.unwrap_or_default());

    let _ = client
        .put_item()
        .table_name("users")
        .item("username", username_av)
        .item("password_hash", password_hash_av)
        .send()
        .await;

    return Some(User {
        username: username.to_string(),
    });
}

pub async fn get_user_by_username(client: &Client, username: &str) -> Option<User> {
    let request = client
        .scan()
        .table_name("users")
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
                        return Some(User {
                            username: username.to_string(),
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
        .table_name("users")
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
