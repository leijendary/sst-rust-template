use lambda_http::{http::header::CONTENT_TYPE, Body, Error, Response};
use serde::Serialize;

use crate::error::result::ErrorResult;

pub fn json_response<T: Serialize>(
    status: u16,
    result: Result<T, ErrorResult>,
) -> Result<Response<Body>, Error> {
    let (status, body) = match result {
        Ok(value) => (status, serde_json::to_string(&value).unwrap()),
        Err(error) => return error_response(error),
    };
    let response = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .status(status)
        .body(body.into())
        .map_err(Box::new)?;

    Ok(response)
}

pub fn error_response(result: ErrorResult) -> Result<Response<Body>, Error> {
    let body = serde_json::to_string(&result).unwrap();
    let response = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .status(result.status)
        .body(body.into())
        .map_err(Box::new)?;

    Ok(response)
}
