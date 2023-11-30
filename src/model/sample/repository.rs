use async_trait::async_trait;
use serde::Serialize;
use serde_with::skip_serializing_none;
use sqlx::{query_as, Error, FromRow};

use rust_decimal::Decimal;
use time::OffsetDateTime;

use crate::{
    database::postgres::PostgresRepository,
    error::{parser::database_error, result::ErrorResult},
    request::{page::PageRequest, seek::SeekRequest},
    response::seek::Seekable,
};

#[skip_serializing_none]
#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleList {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub amount: Decimal,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified_at: OffsetDateTime,
}

impl Seekable for SampleList {
    fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    fn id(&self) -> i64 {
        self.id
    }
}

#[skip_serializing_none]
#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleDetail {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub amount: Decimal,
    pub version: u16,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub created_by: String,
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified_at: OffsetDateTime,
    pub last_modified_by: String,
}

pub struct SampleSeekFilter {
    pub language: Option<String>,
    pub query: Option<String>,
}

#[async_trait]
pub trait SampleRepository {
    async fn sample_seek(
        &self,
        filter: &SampleSeekFilter,
        seekable: &SeekRequest,
    ) -> Result<Vec<SampleList>, ErrorResult>;

    async fn sample_list(
        &self,
        query: &Option<String>,
        page_request: &PageRequest,
    ) -> Result<Vec<SampleList>, ErrorResult>;

    async fn sample_count(&self, query: &Option<String>) -> Result<i64, ErrorResult>;
}

#[async_trait]
impl SampleRepository for PostgresRepository {
    async fn sample_seek(
        &self,
        filter: &SampleSeekFilter,
        seekable: &SeekRequest,
    ) -> Result<Vec<SampleList>, ErrorResult> {
        let sql = "select s.id, t.name, t.description, amount, created_at, last_modified_at
            from sample s
            left join lateral (
                select name, description
                from sample_translation
                where id = s.id
                order by (language = $1)::int desc, ordinal
                limit 1
            ) t on true
            where
                deleted_at is null
                and (s.name ilike concat('%%', $2::text, '%%') or t.name ilike concat('%%', $2::text, '%%'))
                and ($4 is null or $5 is null or (created_at, id) < ($4, $5))
            order by created_at desc, id desc
	        limit $3";
        let result = query_as::<_, SampleList>(sql)
            .bind(&filter.language)
            .bind(&filter.query)
            .bind(seekable.limit())
            .bind(seekable.created_at)
            .bind(seekable.id)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(error) => Err(database_error(error)),
        }
    }

    async fn sample_list(
        &self,
        query: &Option<String>,
        page_request: &PageRequest,
    ) -> Result<Vec<SampleList>, ErrorResult> {
        let sql = "select id, name, description, amount, created_at, last_modified_at
            from sample
            where deleted_at is null and name ilike concat('%%', $1::text, '%%')
            order by created_at desc
            limit $2
            offset $3";
        let result = query_as::<_, SampleList>(sql)
            .bind(query)
            .bind(page_request.limit())
            .bind(page_request.offset())
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(error) => Err(database_error(error)),
        }
    }

    async fn sample_count(&self, query: &Option<String>) -> Result<i64, ErrorResult> {
        let sql = "select count(*)
            from sample
            where deleted_at is null and name ilike concat('%%', $1::text, '%%')";
        let result: Result<(i64,), Error> = query_as(sql).bind(query).fetch_one(&self.pool).await;

        match result {
            Ok(result) => Ok(result.0),
            Err(error) => Err(database_error(error)),
        }
    }
}
