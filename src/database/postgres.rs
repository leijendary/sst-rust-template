use std::env;

use aws_sdk_secretsmanager::Client;
use sqlx::{postgres::PgPoolOptions, PgPool};

const DATABASE_URL: &str = "DATABASE_URL";

pub struct PostgresRepository {
    pub pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub async fn connect_postgres(client: &Client) -> PgPool {
    let url = match env::var_os(DATABASE_URL) {
        Some(url) => url.into_string().unwrap(),
        None => url_from_secret(client).await,
    };

    PgPoolOptions::new()
        .max_connections(2)
        .connect(url.as_str())
        .await
        .expect("Unable to connect to PostgreSQL")
}

async fn url_from_secret(client: &Client) -> String {
    let prefix = env::var("SST_SSM_PREFIX").expect("SST_SSM_PREFIX is not set");
    let key = format!("{prefix}Secret/{DATABASE_URL}/value");
    let value = client
        .get_secret_value()
        .secret_id(key)
        .send()
        .await
        .unwrap();

    value.secret_string().unwrap().to_string()
}
