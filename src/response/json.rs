use lambda_http::{http::header::CONTENT_TYPE, Body, Error, Response};
use serde::Serialize;
use tracing::error;

use crate::error::result::{internal_server, ErrorResult};

pub fn json_response<T: Serialize>(
    status: u16,
    result: Result<T, ErrorResult>,
) -> Result<Response<Body>, Error> {
    match result.and_then(|value| to_json(&value, "json_response")) {
        Ok(json) => build_response(status, json),
        Err(error) => error_response(error),
    }
}

pub fn error_response(result: ErrorResult) -> Result<Response<Body>, Error> {
    match to_json(&result, "error_response") {
        Ok(json) => build_response(result.status, json),
        Err(error) => return error_response(error),
    }
}

fn to_json<T: Serialize>(value: &T, target: &str) -> Result<String, ErrorResult> {
    match serde_json::to_string(value) {
        Ok(json) => Ok(json),
        Err(error) => {
            error!(target, "Error when parsing the struct. {:?}", error);
            Err(internal_server())
        }
    }
}

fn build_response(status: u16, json: String) -> Result<Response<Body>, Error> {
    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .status(status)
        .body(json.into())
        .map(|res| Ok(res))?
}
