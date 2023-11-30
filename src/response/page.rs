use serde::Serialize;

use crate::request::page::PageRequest;

#[derive(Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub size: i16,
    pub total: i64,
}

impl<T> Page<T> {
    pub fn new(data: Vec<T>, count: i64, page_request: &PageRequest) -> Page<T> {
        Page {
            data,
            page: page_request.page,
            size: page_request.size,
            total: count,
        }
    }
}
