use model::page::PageRequest;

use crate::query::query_param;
use lambda_http::Request;

const PAGE_DEFAULT: i64 = 1;
const SIZE_DEFAULT: i16 = 20;

pub trait ApiPageRequest {
    fn read(request: &Request) -> PageRequest;
}

impl ApiPageRequest for PageRequest {
    fn read(request: &Request) -> PageRequest {
        let page = query_param(request, "page").unwrap_or(PAGE_DEFAULT);
        let size = query_param(request, "size").unwrap_or(SIZE_DEFAULT);
        let offset = ((page - 1) * (size as i64)).max(0);

        PageRequest { page, size, offset }
    }
}
