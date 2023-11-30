use tokio::try_join;

use crate::{
    database::repository::PostgresRepository, error::result::ErrorResult, request::page::Pageable,
    response::page::Page,
};

use super::repository::{SampleList, SampleRepository};

pub struct SampleService {
    pub repository: PostgresRepository,
}

impl SampleService {
    pub async fn list(
        &self,
        query: String,
        pageable: Pageable,
    ) -> Result<Page<SampleList>, ErrorResult> {
        let result = try_join!(
            self.repository.sample_list(&query, &pageable),
            self.repository.sample_count(&query)
        );

        match result {
            Ok((list, count)) => Ok(Page::new(list, count, &pageable)),
            Err(error) => Err(error),
        }
    }
}
