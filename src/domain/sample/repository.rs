use crate::{
    database::postgres::PostgresRepository,
    error::{
        parser::{database_error, resource_error},
        result::{version_conflict, ErrorResult},
    },
    request::{page::PageRequest, seek::SeekRequest},
};
use async_trait::async_trait;
use sqlx::{query, query_as, Postgres, Transaction};

use super::model::{SampleDetail, SampleList, SampleRequest, SampleSeekFilter, SampleTranslation};

const POINTER: &'static str = "/data/sample";

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
        sample: &SampleRequest,
    ) -> Result<SampleDetail, ErrorResult>;

    async fn sample_get(
        &self,
        id: i64,
        translate: bool,
        language: &Option<String>,
    ) -> Result<SampleDetail, ErrorResult>;

    async fn sample_update(
        &self,
        tx: &mut Transaction<Postgres>,
        id: i64,
        sample: &SampleRequest,
        version: i16,
    ) -> Result<SampleDetail, ErrorResult>;

    async fn sample_delete(
        &self,
        id: i64,
        version: i16,
        user_id: String,
    ) -> Result<(), ErrorResult>;

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

    async fn sample_translations_update(
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
        sample: &SampleRequest,
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
            .map_err(|error| resource_error(id, POINTER, None, error))
    }

    async fn sample_update(
        &self,
        tx: &mut Transaction<Postgres>,
        id: i64,
        sample: &SampleRequest,
        version: i16,
    ) -> Result<SampleDetail, ErrorResult> {
        let sql = "update sample
            set
                name = $3,
                description = $4,
                amount = $5,
                version = version + 1,
                last_modified_at = now(),
                last_modified_by = $6
            where id = $1 and version = $2
            returning id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by";

        query_as(sql)
            .bind(id)
            .bind(version)
            .bind(&sample.name)
            .bind(&sample.description)
            .bind(&sample.amount)
            .bind(&sample.last_modified_by)
            .fetch_one(&mut **tx)
            .await
            .map_err(|error| resource_error(id, POINTER, Some(version), error))
    }

    async fn sample_delete(
        &self,
        id: i64,
        version: i16,
        user_id: String,
    ) -> Result<(), ErrorResult> {
        let sql = "update sample
            set
                version = version + 1,
                deleted_by = $3,
                deleted_at = now()
            where id = $1 and version = $2";
        let result = query(sql)
            .bind(id)
            .bind(version)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|error| resource_error(id, POINTER, Some(version), error))?;

        if result.rows_affected() == 0 {
            return Err(version_conflict(id, POINTER, version));
        }

        Ok(())
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
            select * from unnest($1::int[], $2::text[], $3::text[], $4::text[], $5::smallint[])
            returning name, description, language, ordinal";
        let (ids, names, descriptions, languages, ordinals) = translations_binds(id, translations);

        query_as(sql)
            .bind(ids)
            .bind(names)
            .bind(descriptions)
            .bind(languages)
            .bind(ordinals)
            .fetch_all(&mut **tx)
            .await
            .map_err(|error| database_error(error))
    }

    async fn sample_translations_update(
        &self,
        tx: &mut Transaction<Postgres>,
        id: i64,
        translations: &Vec<SampleTranslation>,
    ) -> Result<Vec<SampleTranslation>, ErrorResult> {
        let mut sql = "delete from sample_translation where id = $1 and language <> all($2)";
        let (ids, names, descriptions, languages, ordinals) = translations_binds(id, translations);

        query(sql)
            .bind(id)
            .bind(&languages)
            .execute(&mut **tx)
            .await
            .map_err(|error| database_error(error))?;

        sql = "insert into sample_translation (id, name, description, language, ordinal)
            select * from unnest($1::int[], $2::text[], $3::text[], $4::text[], $5::smallint[])
            on conflict (id, language)
            do update
            set
                name = excluded.name,
                description = excluded.description,
                language = excluded.language,
                ordinal = excluded.ordinal
            returning name, description, language, ordinal";

        query_as(sql)
            .bind(ids)
            .bind(names)
            .bind(descriptions)
            .bind(languages)
            .bind(ordinals)
            .fetch_all(&mut **tx)
            .await
            .map_err(|error| database_error(error))
    }
}

fn translations_binds(
    id: i64,
    translations: &Vec<SampleTranslation>,
) -> (
    Vec<i64>,
    Vec<String>,
    Vec<Option<String>>,
    Vec<String>,
    Vec<i16>,
) {
    let len = translations.len();
    let ids = vec![id; len];
    let mut names: Vec<String> = Vec::with_capacity(len);
    let mut descriptions: Vec<Option<String>> = Vec::with_capacity(len);
    let mut languages: Vec<String> = Vec::with_capacity(len);
    let mut ordinals: Vec<i16> = Vec::with_capacity(len);

    for translation in translations {
        names.push(translation.name.to_owned());
        descriptions.push(translation.description.to_owned());
        languages.push(translation.language.to_owned());
        ordinals.push(translation.ordinal);
    }

    return (ids, names, descriptions, languages, ordinals);
}
