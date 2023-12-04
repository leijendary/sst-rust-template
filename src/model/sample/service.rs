use tokio::try_join;

use crate::{
    database::postgres::{begin, commit, PostgresRepository},
    error::result::ErrorResult,
    request::{page::PageRequest, seek::SeekRequest},
    response::{page::Page, seek::Seek},
};

use super::repository::{
    SampleDetail, SampleList, SampleRepository, SampleRequest, SampleSeekFilter,
};

pub struct SampleService {
    pub repository: PostgresRepository,
}

impl SampleService {
    pub async fn seek(
        &self,
        filter: &SampleSeekFilter,
        seek_request: &SeekRequest,
    ) -> Result<Seek<SampleList>, ErrorResult> {
        let list = self.repository.sample_seek(filter, seek_request).await?;

        Ok(Seek::new(list, seek_request))
    }

    pub async fn list(
        &self,
        query: &Option<String>,
        page_request: &PageRequest,
    ) -> Result<Page<SampleList>, ErrorResult> {
        let (list, count) = try_join!(
            self.repository.sample_list(query, page_request),
            self.repository.sample_count(query)
        )?;

        Ok(Page::new(list, count, &page_request))
    }

    pub async fn create(&self, request: &SampleRequest) -> Result<SampleDetail, ErrorResult> {
        let translations = request.translations.as_ref().unwrap();
        let mut tx = begin(&self.repository.pool).await?;
        let mut sample = self.repository.sample_create(&mut tx, request).await?;
        sample.translations = self
            .repository
            .sample_translations_create(&mut tx, sample.id, translations)
            .await
            .map(|translations| Some(translations))?;

        commit(tx).await?;

        Ok(sample)
    }

    /// Gets a single sample record and returns the result.
    /// If `translate` is true, `language` will be used to get the first translation applicable.
    pub async fn get(
        &self,
        id: i64,
        translate: bool,
        language: &Option<String>,
    ) -> Result<SampleDetail, ErrorResult> {
        let sample_fut = self.repository.sample_get(id, translate, language);

        if translate {
            return sample_fut.await;
        }

        let translations_fut = self.repository.sample_translations_list(id);
        let (mut sample, translations) = try_join!(sample_fut, translations_fut)?;
        sample.translations = Some(translations);

        Ok(sample)
    }

    pub async fn update(
        &self,
        id: i64,
        request: &SampleRequest,
        version: i16,
    ) -> Result<SampleDetail, ErrorResult> {
        let translations = request.translations.as_ref().unwrap();
        let mut tx = begin(&self.repository.pool).await?;
        let mut sample = self
            .repository
            .sample_update(&mut tx, id, request, version)
            .await?;
        sample.translations = self
            .repository
            .sample_translations_update(&mut tx, id, translations)
            .await
            .map(|translations| Some(translations))?;

        commit(tx).await?;

        Ok(sample)
    }

    pub async fn delete(&self, id: i64, version: i16, user_id: String) -> Result<(), ErrorResult> {
        self.repository.sample_delete(id, version, user_id).await
    }
}
