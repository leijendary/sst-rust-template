use lambda_http::Request;

use super::query::query_param;

const PAGE_DEFAULT: i64 = 1;
const SIZE_DEFAULT: i16 = 20;

pub struct PageRequest {
    pub page: i64,
    pub size: i16,
}

impl PageRequest {
    pub fn new(request: &Request) -> PageRequest {
        let page = query_param(request, "page").unwrap_or(PAGE_DEFAULT);
        let size = query_param(request, "size").unwrap_or(SIZE_DEFAULT);

        PageRequest { page, size }
    }

    pub fn limit(&self) -> i16 {
        self.size
    }

    pub fn offset(&self) -> i64 {
        ((self.page - 1) * (self.size as i64)).max(0)
    }
}
