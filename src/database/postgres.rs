use std::env;

use aws_sdk_secretsmanager::Client;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};

use crate::error::{parser::database_error, result::ErrorResult};

const DATABASE_URL: &str = "DATABASE_URL";

pub async fn connect_postgres(client: &Client, max: u32) -> PgPool {
    let url = match env::var_os(DATABASE_URL) {
        Some(url) => url.into_string().expect("DATABASE_URL is not set"),
        None => url_from_secret(client).await,
    };

    PgPoolOptions::new()
        .max_connections(max)
        .connect(&url)
        .await
        .expect("Unable to connect to PostgreSQL")
}

pub async fn begin(pool: &PgPool) -> Result<Transaction<Postgres>, ErrorResult> {
    pool.begin().await.map_err(database_error)
}

pub async fn commit(tx: Transaction<'_, Postgres>) -> Result<(), ErrorResult> {
    tx.commit().await.map_err(database_error)
}

pub async fn rollback(tx: Transaction<'_, Postgres>) -> Result<(), ErrorResult> {
    tx.rollback().await.map_err(database_error)
}

async fn url_from_secret(client: &Client) -> String {
    let prefix = env::var("SST_SSM_PREFIX").expect("SST_SSM_PREFIX is not set");
    let key = format!("{prefix}Secret/{DATABASE_URL}/value");

    client
        .get_secret_value()
        .secret_id(key)
        .send()
        .await
        .expect("SST_SSM_PREFIX could not be retrieved")
        .secret_string()
        .expect("DATABASE_URL is not set from SSM")
        .to_string()
}
