use aws_sdk_dynamodb::Client;
use aws_config::{load_defaults, BehaviorVersion};
use tokio;

pub mod auth;
pub mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_config = load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);

    let tables = client.list_tables().send().await?;
    println!("{:?}", tables.table_names);
    Ok(())
}
