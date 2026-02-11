use aws_config::load_from_env;
use aws_sdk_dynamodb::config;

pub mod auth;
pub mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);

    let tables = client.list_tables().send().await?;
    println!("{:?}", tables.table_names);
    Ok(())
}
