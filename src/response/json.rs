use lambda_http::{Body, Error, Response};
use serde::Serialize;

use crate::error::result::ErrorResult;

pub fn json_response<T: Serialize>(
    status: u16,
    result: Result<T, ErrorResult>,
) -> Result<Response<Body>, Error> {
    let (status, body) = match result {
        Ok(value) => (status, serde_json::to_string(&value).unwrap()),
        Err(error) => (error.status, serde_json::to_string(&error).unwrap()),
    };
    let response = Response::builder()
        .header("Content-Type", "application/json")
        .status(status)
        .body(body.into())
        .map_err(Box::new)?;

    Ok(response)
}
