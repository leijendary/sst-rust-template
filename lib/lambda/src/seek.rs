use crate::request::RequestExtension;
use lambda_http::Request;
use model::seek::SeekRequest;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const SIZE_DEFAULT: i16 = 20;
const SIZE_MIN: i16 = 1;

pub trait ApiSeekRequest {
    fn read(request: &Request) -> SeekRequest;
}

impl ApiSeekRequest for SeekRequest {
    fn read(request: &Request) -> SeekRequest {
        let size = request
            .query_param("size")
            .unwrap_or(SIZE_DEFAULT)
            .max(SIZE_MIN);
        let created_at = request
            .query_param::<String>("createdAt")
            .and_then(|value| OffsetDateTime::parse(&value, &Rfc3339).ok());
        let id = request.query_param("id");

        SeekRequest {
            size,
            limit: size + 1,
            created_at,
            id,
        }
    }
}
