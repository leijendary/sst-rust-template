use lambda_http::Request;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::query::query_param;

const SIZE_DEFAULT: i16 = 20;

pub struct SeekRequest {
    pub size: i16,
    pub created_at: Option<OffsetDateTime>,
    pub id: Option<i64>,
}

impl SeekRequest {
    pub fn new(request: &Request) -> SeekRequest {
        let size = query_param(request, "size").unwrap_or(SIZE_DEFAULT);
        let created_at = match query_param::<String>(request, "createdAt") {
            Some(value) => match OffsetDateTime::parse(&value, &Rfc3339) {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            None => None,
        };
        let id = query_param(request, "id");

        SeekRequest {
            size,
            created_at,
            id,
        }
    }

    pub fn limit(&self) -> i16 {
        self.size + 1
    }
}
