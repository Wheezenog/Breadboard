use aws_sdk_dynamodb::{Client, types::AttributeValue};

pub async fn verify_username(client: &Client, username: &str) -> bool {
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
                return !items.is_empty();
            }
            false
        }
        Err(_) => false,
    }
}

pub async fn verify_password_strength(password: &str) -> bool {
    // i really dont want to do anything fancy
    // make your password 8 characters brah
    password.len() <= 8
}
