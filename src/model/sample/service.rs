use tokio::try_join;

use crate::{
    database::postgres::PostgresRepository,
    error::result::ErrorResult,
    request::{page::PageRequest, seek::SeekRequest},
    response::{page::Page, seek::Seek},
};

use super::repository::{SampleList, SampleRepository, SampleSeekFilter};

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

        Ok(Seek::new(list, &seekable))
    }

    pub async fn list(
        &self,
        query: &Option<String>,
        page_request: &PageRequest,
    ) -> Result<Page<SampleList>, ErrorResult> {
        let (list, count) = try_join!(
            self.repository.sample_list(&query, &page_request),
            self.repository.sample_count(&query)
        )?;

        Ok(Page::new(list, count, &page_request))
    }
}
