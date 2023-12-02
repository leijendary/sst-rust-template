use tokio::try_join;

use crate::{
    database::postgres::{begin, commit, PostgresRepository},
    error::result::ErrorResult,
    request::{page::PageRequest, seek::SeekRequest},
    response::{page::Page, seek::Seek},
};

use super::repository::{
    SampleCreate, SampleDetail, SampleList, SampleRepository, SampleSeekFilter,
};

pub struct SampleService {
    pub repository: PostgresRepository,
}

impl SampleService {
    pub async fn seek(
        &self,
        filter: &SampleSeekFilter,
        seekable: &SeekRequest,
    ) -> Result<Seek<SampleList>, ErrorResult> {
        let list = self.repository.sample_seek(filter, seekable).await?;

        Ok(Seek::new(list, seekable))
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

    pub async fn create(&self, request: &SampleCreate) -> Result<SampleDetail, ErrorResult> {
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

    pub async fn get(&self, id: i64) -> Result<SampleDetail, ErrorResult> {
        let (mut sample, translations) = try_join!(
            self.repository.sample_get(id),
            self.repository.sample_translations_list(id)
        )?;
        sample.translations = Some(translations);

        Ok(sample)
    }
}
