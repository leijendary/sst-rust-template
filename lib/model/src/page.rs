use serde::Serialize;

pub struct PageRequest {
    pub page: i64,
    pub size: i16,
    pub offset: i64,
}

#[derive(Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub size: i16,
    pub total: i64,
}

impl<T> Page<T> {
    pub fn new(data: Vec<T>, count: i64, page_request: &PageRequest) -> Self {
        Self {
            data,
            page: page_request.page,
            size: page_request.size,
            total: count,
        }
    }
}
