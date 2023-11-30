use lambda_http::Request;

use super::query::query_param;

pub struct Pageable {
    pub page: i64,
    pub size: i16,
}

const PAGE_DEFAULT: i64 = 1;
const SIZE_DEFAULT: i16 = 20;

impl Pageable {
    pub fn new(request: Request) -> Pageable {
        let page = query_param(&request, "page", PAGE_DEFAULT);
        let size = query_param(&request, "size", SIZE_DEFAULT);

        Pageable { page, size }
    }

    pub fn limit(&self) -> i16 {
        self.size
    }

    pub fn offset(&self) -> i64 {
        (self.page - 1) * (self.size as i64)
    }
}
