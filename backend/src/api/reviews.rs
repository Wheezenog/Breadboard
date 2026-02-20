use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde_dynamo::from_items;

use crate::types::Review;

pub async fn get_all_reviews(State(client): State<Arc<Client>>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = client
        .scan()
        .table_name("Reviews")
        .limit(15)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(items) = result.items {
        let reviews: Vec<Review> =
            from_items(items).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let json_output =
            serde_json::to_string_pretty(&reviews).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok((StatusCode::OK, json_output))
    } else {
        Err((StatusCode::NOT_FOUND, "No reviews found".to_string()))
    }
}
