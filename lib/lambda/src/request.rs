use lambda_http::{Request, RequestPayloadExt};
use model::error::{invalid_body, required_body, ErrorResult};
use serde::de::DeserializeOwned;

pub trait RequestPayloadParser {
    fn parse_payload<D: DeserializeOwned>(&self) -> Result<D, ErrorResult>;
}

impl RequestPayloadParser for Request {
    fn parse_payload<D: DeserializeOwned>(&self) -> Result<D, ErrorResult> {
        match self.payload::<D>() {
            Ok(value) => value.ok_or_else(required_body),
            Err(_) => Err(invalid_body()),
        }
    }
}
