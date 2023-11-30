use async_trait::async_trait;
use serde::Serialize;
use sqlx::{query_as, Error, FromRow};

use rust_decimal::Decimal;
use time::OffsetDateTime;

use crate::{
    database::repository::PostgresRepository,
    error::{parser::database_error, result::ErrorResult},
    request::page::Pageable,
};

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleList {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: Decimal,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified_at: OffsetDateTime,
}

pub struct SampleDetail {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub amount: Decimal,
    pub version: u16,
    pub created_at: OffsetDateTime,
    pub created_by: String,
    pub last_modified_at: OffsetDateTime,
    pub last_modified_by: String,
}

#[async_trait]
pub trait SampleRepository {
    async fn sample_list(
        &self,
        query: &String,
        pageable: &Pageable,
    ) -> Result<Vec<SampleList>, ErrorResult>;

    async fn sample_count(&self, query: &String) -> Result<i64, ErrorResult>;
}

#[async_trait]
impl SampleRepository for PostgresRepository {
    async fn sample_list(
        &self,
        query: &String,
        pageable: &Pageable,
    ) -> Result<Vec<SampleList>, ErrorResult> {
        let sql = "select id, name, description, amount, created_at, last_modified_at
            from sample
            where name ilike concat('%%', $1::text, '%%') and deleted_at is null
            order by created_at desc
            limit $2
            offset $3";
        let result = query_as::<_, SampleList>(sql)
            .bind(query)
            .bind(pageable.limit())
            .bind(pageable.offset())
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(error) => Err(database_error(error)),
        }
    }

    async fn sample_count(&self, query: &String) -> Result<i64, ErrorResult> {
        let sql = "select count(*)
            from sample
            where name ilike concat('%%', $1::text, '%%') and deleted_at is null";
        let result: Result<(i64,), Error> = query_as(sql).bind(query).fetch_one(&self.pool).await;

        match result {
            Ok(result) => Ok(result.0),
            Err(error) => Err(database_error(error)),
        }
    }
}
