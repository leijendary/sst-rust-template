use tokio::try_join;

use database::postgres::{begin, commit};
use model::{
    error::ErrorResult,
    page::{Page, PageRequest},
    seek::{Seek, SeekRequest},
};

use super::{
    model::{SampleDetail, SampleList, SampleRequest, SampleSeekFilter},
    repository::SampleRepository,
};

pub struct SampleService {
    pub repository: SampleRepository,
}

impl SampleService {
    pub async fn seek(
        &self,
        filter: &SampleSeekFilter,
        seek_request: &SeekRequest,
    ) -> Result<Seek<SampleList>, ErrorResult> {
        let list = self.repository.seek(filter, seek_request).await?;

        Ok(Seek::new(list, seek_request))
    }

    pub async fn page(
        &self,
        query: &Option<String>,
        page_request: &PageRequest,
    ) -> Result<Page<SampleList>, ErrorResult> {
        let (list, count) = try_join!(
            self.repository.page(query, page_request),
            self.repository.count(query)
        )?;

        Ok(Page::new(list, count, page_request))
    }

    pub async fn create(
        &self,
        request: SampleRequest,
        user_id: String,
    ) -> Result<SampleDetail, ErrorResult> {
        let mut tx = begin(&self.repository.db).await?;
        let mut sample = self.repository.create(&mut tx, &request, user_id).await?;
        sample.translations = self
            .repository
            .create_translations(&mut tx, sample.id, request.translations)
            .await
            .map(Some)?;

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
        let sample_fut = self.repository.get(id, translate, language);

        if translate {
            return sample_fut.await;
        }

        let translations_fut = self.repository.list_translations(id);
        let (mut sample, translations) = try_join!(sample_fut, translations_fut)?;
        sample.translations = Some(translations);

        Ok(sample)
    }

    pub async fn update(
        &self,
        id: i64,
        request: SampleRequest,
        version: i16,
        user_id: String,
    ) -> Result<SampleDetail, ErrorResult> {
        let mut tx = begin(&self.repository.db).await?;
        let mut sample = self
            .repository
            .update(&mut tx, id, &request, version, user_id)
            .await?;
        sample.translations = self
            .repository
            .update_translations(&mut tx, id, request.translations)
            .await
            .map(Some)?;

        commit(tx).await?;

        Ok(sample)
    }

    pub async fn delete(&self, id: i64, version: i16, user_id: String) -> Result<(), ErrorResult> {
        self.repository.delete(id, version, user_id).await
    }
}
