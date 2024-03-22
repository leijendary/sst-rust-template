use database::error_parser::{database_error, resource_error};
use model::{
    error::{version_conflict, ErrorResult},
    page::PageRequest,
    seek::SeekRequest,
};
use sqlx::{query, query_as, PgPool, Postgres, Transaction};

use super::model::{SampleDetail, SampleList, SampleRequest, SampleSeekFilter, SampleTranslation};

static ENTITY: &str = "sample";

struct TranslationsBinds {
    ids: Vec<i64>,
    names: Vec<String>,
    descriptions: Vec<Option<String>>,
    languages: Vec<String>,
    ordinals: Vec<i16>,
}

pub struct SampleRepository {
    pub pool: PgPool,
}

impl SampleRepository {
    /// Seek / keyset pagination.
    /// This is a more optimized way to do pagination compared to limit-offset pagination.
    /// The way this works is that this will use indices to get the first n results and
    /// just limits after that.
    pub async fn seek(
        &self,
        filter: &SampleSeekFilter,
        seek_request: &SeekRequest,
    ) -> Result<Vec<SampleList>, ErrorResult> {
        static SQL: &str = include_str!("sql/seek.sql");

        query_as::<_, SampleList>(SQL)
            .bind(&filter.language)
            .bind(&filter.query)
            .bind(seek_request.limit)
            .bind(seek_request.created_at)
            .bind(seek_request.id)
            .fetch_all(&self.pool)
            .await
            .map_err(database_error)
    }

    pub async fn page(
        &self,
        query: &Option<String>,
        page_request: &PageRequest,
    ) -> Result<Vec<SampleList>, ErrorResult> {
        static SQL: &str = include_str!("sql/page.sql");

        query_as::<_, SampleList>(SQL)
            .bind(query)
            .bind(page_request.size)
            .bind(page_request.offset)
            .fetch_all(&self.pool)
            .await
            .map_err(database_error)
    }

    pub async fn count(&self, query: &Option<String>) -> Result<i64, ErrorResult> {
        static SQL: &str = include_str!("sql/count.sql");

        query_as(SQL)
            .bind(query)
            .fetch_one(&self.pool)
            .await
            .map(|result: (i64,)| result.0)
            .map_err(database_error)
    }

    pub async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        sample: &SampleRequest,
        user_id: String,
    ) -> Result<SampleDetail, ErrorResult> {
        static SQL: &str = include_str!("sql/create.sql");

        query_as::<_, SampleDetail>(SQL)
            .bind(&sample.name)
            .bind(&sample.description)
            .bind(sample.amount)
            .bind(&user_id)
            .bind(&user_id)
            .fetch_one(&mut **tx)
            .await
            .map_err(database_error)
    }

    pub async fn get(
        &self,
        id: i64,
        translate: bool,
        language: &Option<String>,
    ) -> Result<SampleDetail, ErrorResult> {
        static SQL: &str = include_str!("sql/get.sql");

        query_as::<_, SampleDetail>(SQL)
            .bind(id)
            .bind(translate)
            .bind(language)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| resource_error(ENTITY, id, None, error))
    }

    pub async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: i64,
        sample: &SampleRequest,
        version: i16,
        user_id: String,
    ) -> Result<SampleDetail, ErrorResult> {
        static SQL: &str = include_str!("sql/update.sql");

        query_as::<_, SampleDetail>(SQL)
            .bind(id)
            .bind(version)
            .bind(&sample.name)
            .bind(&sample.description)
            .bind(sample.amount)
            .bind(user_id)
            .fetch_one(&mut **tx)
            .await
            .map_err(|error| resource_error(ENTITY, id, Some(version), error))
    }

    pub async fn delete(&self, id: i64, version: i16, user_id: String) -> Result<(), ErrorResult> {
        static SQL: &str = include_str!("sql/delete.sql");
        let result = query(SQL)
            .bind(id)
            .bind(version)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|error| resource_error(ENTITY, id, Some(version), error))?;

        if result.rows_affected() == 0 {
            return Err(version_conflict(ENTITY, id, version));
        }

        Ok(())
    }

    pub async fn list_translations(&self, id: i64) -> Result<Vec<SampleTranslation>, ErrorResult> {
        static SQL: &str = include_str!("sql/translations_list.sql");

        query_as::<_, SampleTranslation>(SQL)
            .bind(id)
            .fetch_all(&self.pool)
            .await
            .map_err(database_error)
    }

    pub async fn create_translations(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: i64,
        translations: Vec<SampleTranslation>,
    ) -> Result<Vec<SampleTranslation>, ErrorResult> {
        static SQL: &str = include_str!("sql/translations_create.sql");
        let binds = translations_binds(id, translations);

        query_as::<_, SampleTranslation>(SQL)
            .bind(binds.ids)
            .bind(binds.names)
            .bind(binds.descriptions)
            .bind(binds.languages)
            .bind(binds.ordinals)
            .fetch_all(&mut **tx)
            .await
            .map_err(database_error)
    }

    pub async fn update_translations(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: i64,
        translations: Vec<SampleTranslation>,
    ) -> Result<Vec<SampleTranslation>, ErrorResult> {
        static SQL_DELETE: &str = include_str!("sql/translations_delete.sql");
        let binds = translations_binds(id, translations);

        query(SQL_DELETE)
            .bind(id)
            .bind(&binds.languages)
            .execute(&mut **tx)
            .await
            .map_err(database_error)?;

        static SQL_UPSERT: &str = include_str!("sql/translations_upsert.sql");

        query_as::<_, SampleTranslation>(SQL_UPSERT)
            .bind(binds.ids)
            .bind(binds.names)
            .bind(binds.descriptions)
            .bind(binds.languages)
            .bind(binds.ordinals)
            .fetch_all(&mut **tx)
            .await
            .map_err(database_error)
    }
}

fn translations_binds(id: i64, translations: Vec<SampleTranslation>) -> TranslationsBinds {
    let len = translations.len();
    let ids = vec![id; len];
    let mut names: Vec<String> = Vec::with_capacity(len);
    let mut descriptions: Vec<Option<String>> = Vec::with_capacity(len);
    let mut languages: Vec<String> = Vec::with_capacity(len);
    let mut ordinals: Vec<i16> = Vec::with_capacity(len);

    for translation in translations {
        names.push(translation.name);
        descriptions.push(translation.description);
        languages.push(translation.language);
        ordinals.push(translation.ordinal);
    }

    TranslationsBinds {
        ids,
        names,
        descriptions,
        languages,
        ordinals,
    }
}
