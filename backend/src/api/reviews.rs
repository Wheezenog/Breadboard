use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{extract::State, response::IntoResponse};

pub async fn get_all_reviews(State(client): State<Arc<Client>>) -> impl IntoResponse {
  
}