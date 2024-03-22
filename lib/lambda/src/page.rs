use model::page::PageRequest;

use crate::query::query_param;
use lambda_http::Request;

const PAGE_DEFAULT: i64 = 1;
const PAGE_MIN: i64 = 1;
const SIZE_DEFAULT: i16 = 20;
const SIZE_MIN: i16 = 1;

pub trait ApiPageRequest {
    fn read(request: &Request) -> PageRequest;
}

impl ApiPageRequest for PageRequest {
    fn read(request: &Request) -> PageRequest {
        let page = query_param(request, "page")
            .unwrap_or(PAGE_DEFAULT)
            .max(PAGE_MIN);
        let size = query_param(request, "size")
            .unwrap_or(SIZE_DEFAULT)
            .max(SIZE_MIN);
        let offset = ((page - 1) * i64::from(size)).max(0);

        PageRequest { page, size, offset }
    }
}

#[cfg(test)]
mod tests {
    use crate::page::{PAGE_MIN, SIZE_MIN};

    use super::ApiPageRequest;
    use lambda_http::{aws_lambda_events::query_map::QueryMap, Request, RequestExt};
    use model::page::PageRequest;
    use std::collections::HashMap;

    #[test]
    fn read_should_return_correct_values() {
        let query_params = HashMap::<String, Vec<String>>::from([
            ("page".into(), vec!["1".into()]),
            ("size".into(), vec!["10".into()]),
        ]);
        let query_map = QueryMap::from(query_params);
        let request = Request::default().with_query_string_parameters(query_map);
        let result = PageRequest::read(&request);

        assert_eq!(result.page, 1);
        assert_eq!(result.size, 10);
        assert_eq!(result.offset, 0);
    }

    #[test]
    fn read_negative_page_should_return_min() {
        let query_params = HashMap::<String, Vec<String>>::from([
            ("page".into(), vec!["-1".into()]),
            ("size".into(), vec!["10".into()]),
        ]);
        let query_map = QueryMap::from(query_params);

        let request = Request::default().with_query_string_parameters(query_map);
        let result = PageRequest::read(&request);

        assert_eq!(result.page, PAGE_MIN);
        assert_eq!(result.size, 10);
        assert_eq!(result.offset, 0);
    }

    #[test]
    fn read_negative_size_should_return_min() {
        let query_params = HashMap::<String, Vec<String>>::from([
            ("page".into(), vec!["1".into()]),
            ("size".into(), vec!["-1".into()]),
        ]);
        let query_map = QueryMap::from(query_params);

        let request = Request::default().with_query_string_parameters(query_map);
        let result = PageRequest::read(&request);

        assert_eq!(result.page, 1);
        assert_eq!(result.size, SIZE_MIN);
        assert_eq!(result.offset, 0);
    }

    #[test]
    fn read_next_page_should_return_offset() {
        let query_params = HashMap::<String, Vec<String>>::from([
            ("page".into(), vec!["2".into()]),
            ("size".into(), vec!["10".into()]),
        ]);
        let query_map = QueryMap::from(query_params);

        let request = Request::default().with_query_string_parameters(query_map);
        let result = PageRequest::read(&request);

        assert_eq!(result.page, 2);
        assert_eq!(result.size, 10);
        assert_eq!(result.offset, 10);
    }
}
