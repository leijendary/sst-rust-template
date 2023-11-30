use serde::Serialize;

use crate::request::page::Pageable;

#[derive(Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub size: i16,
    pub total: i64,
}

impl<T> Page<T> {
    pub fn new(data: Vec<T>, count: i64, pageable: &Pageable) -> Page<T> {
        Page {
            data,
            page: pageable.page,
            size: pageable.size,
            total: count,
        }
    }
}
