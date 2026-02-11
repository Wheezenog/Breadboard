use aws_config::{BehaviorVersion, load_defaults};
use aws_sdk_dynamodb::Client;
use axum::{
    Router,
    extract::State,
    http::{
        HeaderValue, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use tokio;
use tower_http::cors::{Any, CorsLayer};

pub mod auth;
pub mod types;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_config = load_defaults(BehaviorVersion::latest()).await;

    let client = Arc::new(Client::new(&shared_config));

    let app = app(client.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn app(client: Arc<Client>) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    Router::new()
        .route("/api", get(|| async { "Hello from the backend!" }))
        .route("/api/tables", get(hello))
        .with_state(client)
        .layer(cors_layer)
}

async fn hello(State(client): State<Arc<Client>>) -> impl IntoResponse {
    match client.list_tables().send().await {
        Ok(tables) => (StatusCode::OK, tables.table_names.unwrap().join(",")).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response(),
    }
}