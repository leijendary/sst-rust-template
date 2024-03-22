use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_secretsmanager::Client;

pub async fn secret_client() -> Client {
    let config = load_defaults(BehaviorVersion::latest()).await;

    Client::new(&config)
}
