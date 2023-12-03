use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_trim::option_string_trim;
use serde_with::skip_serializing_none;
use sqlx::{query_as, FromRow, Postgres, Transaction};
use time::OffsetDateTime;
use validator::Validate;

use crate::{
    database::postgres::PostgresRepository,
    error::{
        parser::{database_error, resource_error},
        result::ErrorResult,
    },
    model::translation::Translation,
    request::{page::PageRequest, seek::SeekRequest, validator::validate_unique_translation},
    response::seek::Seekable,
};

pub struct SampleSeekFilter {
    pub language: Option<String>,
    pub query: Option<String>,
}

#[skip_serializing_none]
#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleList {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub amount: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl Seekable for SampleList {
    fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    fn id(&self) -> i64 {
        self.id
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct SampleCreate {
    #[serde(default, deserialize_with = "option_string_trim")]
    #[validate(required, length(min = 1, max = 100))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "option_string_trim")]
    pub description: Option<String>,
    #[validate(required, range(min = 1, max = 99999999))]
    pub amount: Option<i32>,
    #[validate(
        required,
        length(min = 1, max = 100),
        custom = "validate_unique_translation"
    )]
    #[validate]
    pub translations: Option<Vec<SampleTranslation>>,
    #[serde(skip)]
    pub created_by: String,
    #[serde(skip)]
    pub last_modified_by: String,
}

#[derive(Debug, FromRow, Validate, Serialize, Deserialize)]
pub struct SampleTranslation {
    #[serde(default, deserialize_with = "option_string_trim")]
    #[validate(required, length(min = 1, max = 100))]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "option_string_trim"
    )]
    #[validate(length(max = 200))]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "option_string_trim")]
    #[validate(required, length(min = 2, max = 4))]
    pub language: Option<String>,
    #[validate(required, range(min = 1, max = 100))]
    pub ordinal: Option<i16>,
}

impl Translation for SampleTranslation {
    fn language(&self) -> String {
        self.language.to_owned().unwrap_or_default()
    }

    fn ordinal(&self) -> i16 {
        self.ordinal.to_owned().unwrap_or_default()
    }
}

#[skip_serializing_none]
#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleDetail {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub amount: i32,
    pub version: i16,
    #[sqlx(skip)]
    pub translations: Option<Vec<SampleTranslation>>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[async_trait]
pub trait SampleRepository {
    /// Seek / keyset pagination.
    /// This is a more optimized way to do pagination compared to limit-offset pagination.
    /// The way this works is that this will use indices to get the first n results and
    /// just limits after that.
    async fn sample_seek(
        &self,
        filter: &SampleSeekFilter,
        seek_request: &SeekRequest,
    ) -> Result<Vec<SampleList>, ErrorResult>;

    async fn sample_list(
        &self,
        query: &Option<String>,
        page_request: &PageRequest,
    ) -> Result<Vec<SampleList>, ErrorResult>;

    async fn sample_count(&self, query: &Option<String>) -> Result<i64, ErrorResult>;

    async fn sample_create(
        &self,
        tx: &mut Transaction<Postgres>,
        sample: &SampleCreate,
    ) -> Result<SampleDetail, ErrorResult>;

    async fn sample_get(
        &self,
        id: i64,
        translate: bool,
        language: &Option<String>,
    ) -> Result<SampleDetail, ErrorResult>;

    async fn sample_translations_list(
        &self,
        id: i64,
    ) -> Result<Vec<SampleTranslation>, ErrorResult>;

    async fn sample_translations_create(
        &self,
        tx: &mut Transaction<Postgres>,
        id: i64,
        translations: &Vec<SampleTranslation>,
    ) -> Result<Vec<SampleTranslation>, ErrorResult>;
}

#[async_trait]
impl SampleRepository for PostgresRepository {
    async fn sample_seek(
        &self,
        filter: &SampleSeekFilter,
        seek_request: &SeekRequest,
    ) -> Result<Vec<SampleList>, ErrorResult> {
        let sql = "select s.id, t.name, t.description, amount, created_at
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

        query_as(sql)
            .bind(&filter.language)
            .bind(&filter.query)
            .bind(seek_request.limit())
            .bind(seek_request.created_at)
            .bind(seek_request.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|error| database_error(error))
    }

    async fn sample_list(
        &self,
        query: &Option<String>,
        page_request: &PageRequest,
    ) -> Result<Vec<SampleList>, ErrorResult> {
        let sql = "select id, name, description, amount, created_at
            from sample
            where deleted_at is null and name ilike concat('%%', $1::text, '%%')
            order by created_at desc
            limit $2
            offset $3";

        query_as(sql)
            .bind(query)
            .bind(page_request.limit())
            .bind(page_request.offset())
            .fetch_all(&self.pool)
            .await
            .map_err(|error| database_error(error))
    }

    async fn sample_count(&self, query: &Option<String>) -> Result<i64, ErrorResult> {
        let sql = "select count(*)
            from sample
            where deleted_at is null and name ilike concat('%%', $1::text, '%%')";

        query_as(sql)
            .bind(query)
            .fetch_one(&self.pool)
            .await
            .map(|result: (i64,)| result.0)
            .map_err(|error| database_error(error))
    }

    async fn sample_create(
        &self,
        tx: &mut Transaction<Postgres>,
        sample: &SampleCreate,
    ) -> Result<SampleDetail, ErrorResult> {
        let sql = "insert into sample (name, description, amount, created_by, last_modified_by)
            values ($1, $2, $3, $4, $5)
            returning id, name, description, amount, version, created_at";

        query_as(sql)
            .bind(&sample.name)
            .bind(&sample.description)
            .bind(&sample.amount)
            .bind(&sample.created_by)
            .bind(&sample.last_modified_by)
            .fetch_one(&mut **tx)
            .await
            .map_err(|error| database_error(error))
    }

    async fn sample_get(
        &self,
        id: i64,
        translate: bool,
        language: &Option<String>,
    ) -> Result<SampleDetail, ErrorResult> {
        let sql = "
            select
                s.id,
                coalesce(t.name, s.name) name,
                coalesce(t.description, s.description) description,
                amount,
                version,
                created_at
            from sample s
            left join lateral (
                select name, description
                from sample_translation
                where id = s.id
                order by (language = $3)::int desc, ordinal
                limit 1
            ) t on $2
            where id = $1 and deleted_at is null";

        query_as(sql)
            .bind(id)
            .bind(translate)
            .bind(language)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| resource_error(id, "/data/sample", error))
    }

    async fn sample_translations_list(
        &self,
        id: i64,
    ) -> Result<Vec<SampleTranslation>, ErrorResult> {
        let sql = "select name, description, language, ordinal
            from sample_translation
            where id = $1";

        query_as(sql)
            .bind(id)
            .fetch_all(&self.pool)
            .await
            .map_err(|error| database_error(error))
    }

    async fn sample_translations_create(
        &self,
        tx: &mut Transaction<Postgres>,
        id: i64,
        translations: &Vec<SampleTranslation>,
    ) -> Result<Vec<SampleTranslation>, ErrorResult> {
        let sql = "insert into sample_translation (id, name, description, language, ordinal)
            select * from unnest ($1::int[], $2::text[], $3::text[], $4::text[], $5::smallint[])
            returning name, description, language, ordinal";
        let len = translations.len();
        let mut ids = Vec::with_capacity(len);
        let mut names = Vec::with_capacity(len);
        let mut descriptions = Vec::with_capacity(len);
        let mut languages = Vec::with_capacity(len);
        let mut ordinals = Vec::with_capacity(len);

        for translation in translations {
            ids.push(id);
            names.push(translation.name.to_owned());
            descriptions.push(translation.description.to_owned());
            languages.push(translation.language.to_owned());
            ordinals.push(translation.ordinal.to_owned());
        }

        query_as(sql)
            .bind(&ids[..])
            .bind(&names as &[Option<String>])
            .bind(&descriptions as &[Option<String>])
            .bind(&languages as &[Option<String>])
            .bind(&ordinals as &[Option<i16>])
            .fetch_all(&mut **tx)
            .await
            .map_err(|error| database_error(error))
    }
}
