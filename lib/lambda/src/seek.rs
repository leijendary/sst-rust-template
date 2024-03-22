use crate::query::query_param;
use lambda_http::Request;
use model::seek::SeekRequest;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const SIZE_DEFAULT: i16 = 20;

pub trait ApiSeekRequest {
    fn read(request: &Request) -> SeekRequest;
}

impl ApiSeekRequest for SeekRequest {
    fn read(request: &Request) -> SeekRequest {
        let size = query_param(request, "size").unwrap_or(SIZE_DEFAULT);
        let limit = size + 1;
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
            limit,
            created_at,
            id,
        }
    }
}
