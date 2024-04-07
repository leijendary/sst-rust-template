use std::future::Future;

use lambda_http::{http::header::CONTENT_TYPE, Body, Error, Response};
use model::error::{internal_server, ErrorResult};
use serde::Serialize;
use tracing::error;

pub async fn json_handler<T, F>(handler: F) -> Result<Response<Body>, Error>
where
    T: Serialize,
    F: Future<Output = Result<(u16, T), ErrorResult>>,
{
    match handler.await {
        Ok((status, value)) => json_response(status, value),
        Err(error) => error_response(error),
    }
}

pub fn json_response<T: Serialize>(status: u16, value: T) -> Result<Response<Body>, Error> {
    to_json(&value, "json_response")
        .map(|json| build_response(status, json))
        .unwrap_or_else(error_response)
}

pub fn error_response(result: ErrorResult) -> Result<Response<Body>, Error> {
    to_json(&result, "error_response")
        .map(|json| build_response(result.status, json))
        .unwrap_or_else(error_response)
}

fn to_json<T: Serialize>(value: &T, target: &str) -> Result<String, ErrorResult> {
    serde_json::to_string(value).map_err(|error| {
        error!(target, "Error when parsing the struct. {:?}", error);
        internal_server()
    })
}

fn build_response(status: u16, json: String) -> Result<Response<Body>, Error> {
    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .status(status)
        .body(json.into())
        .map(Ok)?
}
