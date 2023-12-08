use crate::{config::serde::serialize_option_offset_date_time, request::seek::SeekRequest};
use serde::Serialize;
use serde_with::skip_serializing_none;
use time::OffsetDateTime;

pub trait Seekable {
    fn created_at(&self) -> OffsetDateTime;
    fn id(&self) -> i64;
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Seek<T> {
    pub data: Vec<T>,
    pub size: i16,
    #[serde(serialize_with = "serialize_option_offset_date_time")]
    pub created_at: Option<OffsetDateTime>,
    pub id: Option<i64>,
}

impl<T: Seekable> Seek<T> {
    pub fn new(mut data: Vec<T>, seek_request: &SeekRequest) -> Seek<T> {
        let (created_at, id) = if data.len() > seek_request.size as usize {
            data.pop();
            data.last()
                .map(|last| (Some(last.created_at()), Some(last.id())))
                .unwrap_or((None, None))
        } else {
            (None, None)
        };

        Seek {
            data,
            size: seek_request.size,
            created_at,
            id,
        }
    }
}
