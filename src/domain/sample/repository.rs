use crate::{
    error::{
        parser::{database_error, resource_error},
        result::{version_conflict, ErrorResult},
    },
    request::{page::PageRequest, seek::SeekRequest},
};
use sqlx::{query, query_as, PgPool, Postgres, Transaction};

use super::model::{SampleDetail, SampleList, SampleRequest, SampleSeekFilter, SampleTranslation};

const POINTER: &str = "/data/sample";

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
        const SQL: &str = "select s.id, t.name, t.description, amount, created_at
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

        query_as::<_, SampleList>(SQL)
            .bind(&filter.language)
            .bind(&filter.query)
            .bind(seek_request.limit())
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
        const SQL: &str = "select id, name, description, amount, created_at
            from sample
            where deleted_at is null and name ilike concat('%%', $1::text, '%%')
            order by created_at desc
            limit $2
            offset $3";

        query_as::<_, SampleList>(SQL)
            .bind(query)
            .bind(page_request.limit())
            .bind(page_request.offset())
            .fetch_all(&self.pool)
            .await
            .map_err(database_error)
    }

    pub async fn count(&self, query: &Option<String>) -> Result<i64, ErrorResult> {
        const SQL: &str = "select count(*)
            from sample
            where deleted_at is null and name ilike concat('%%', $1::text, '%%')";

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
    ) -> Result<SampleDetail, ErrorResult> {
        const SQL: &str = "insert into
            sample (name, description, amount, created_by, last_modified_by)
            values ($1, $2, $3, $4, $5)
            returning id, name, description, amount, version, created_at";

        query_as::<_, SampleDetail>(SQL)
            .bind(&sample.name)
            .bind(&sample.description)
            .bind(sample.amount)
            .bind(&sample.created_by)
            .bind(&sample.last_modified_by)
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
        const SQL: &str = "select
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

        query_as::<_, SampleDetail>(SQL)
            .bind(id)
            .bind(translate)
            .bind(language)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| resource_error(id, POINTER, None, error))
    }

    pub async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: i64,
        sample: &SampleRequest,
        version: i16,
    ) -> Result<SampleDetail, ErrorResult> {
        const SQL: &str = "update sample
            set
                name = $3,
                description = $4,
                amount = $5,
                version = version + 1,
                last_modified_at = now(),
                last_modified_by = $6
            where id = $1 and version = $2
            returning id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by";

        query_as::<_, SampleDetail>(SQL)
            .bind(id)
            .bind(version)
            .bind(&sample.name)
            .bind(&sample.description)
            .bind(sample.amount)
            .bind(&sample.last_modified_by)
            .fetch_one(&mut **tx)
            .await
            .map_err(|error| resource_error(id, POINTER, Some(version), error))
    }

    pub async fn delete(&self, id: i64, version: i16, user_id: String) -> Result<(), ErrorResult> {
        const SQL: &str = "update sample
            set version = version + 1, deleted_by = $3, deleted_at = now()
            where id = $1 and version = $2";
        let result = query(SQL)
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

    pub async fn list_translations(&self, id: i64) -> Result<Vec<SampleTranslation>, ErrorResult> {
        const SQL: &str = "select name, description, language, ordinal
            from sample_translation
            where id = $1";

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
        const SQL: &str = "insert into
            sample_translation (id, name, description, language, ordinal)
            select * from unnest($1::int[], $2::text[], $3::text[], $4::text[], $5::smallint[])
            returning name, description, language, ordinal";
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
        const DELETE_SQL: &str = "delete from sample_translation
            where id = $1 and language <> all($2)";
        let binds = translations_binds(id, translations);

        query(DELETE_SQL)
            .bind(id)
            .bind(&binds.languages)
            .execute(&mut **tx)
            .await
            .map_err(database_error)?;

        const INSERT_SQL: &str = "insert into
            sample_translation (id, name, description, language, ordinal)
            select * from unnest($1::int[], $2::text[], $3::text[], $4::text[], $5::smallint[])
            on conflict (id, language)
            do update
            set
                name = excluded.name,
                description = excluded.description,
                language = excluded.language,
                ordinal = excluded.ordinal
            returning name, description, language, ordinal";

        query_as::<_, SampleTranslation>(INSERT_SQL)
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
