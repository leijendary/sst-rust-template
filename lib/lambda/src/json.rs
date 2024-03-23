use lambda_http::{http::header::CONTENT_TYPE, Body, Error, Request, Response};
use model::error::{internal_server, ErrorResult};
use serde::Serialize;
use tracing::error;

static HEADER_AMZN_TRACE_ID: &str = "x-amzn-trace-id";
static HEADER_TRACE_ID: &str = "x-trace-id";

pub fn json_response<T: Serialize>(
    request: Request,
    status: u16,
    result: Result<T, ErrorResult>,
) -> Result<Response<Body>, Error> {
    match result.and_then(|value| to_json(&value, "json_response")) {
        Ok(json) => build_response(request, status, json),
        Err(error) => error_response(request, error),
    }
}

pub fn error_response(request: Request, result: ErrorResult) -> Result<Response<Body>, Error> {
    match to_json(&result, "error_response") {
        Ok(json) => build_response(request, result.status, json),
        Err(error) => error_response(request, error),
    }
}

fn to_json<T: Serialize>(value: &T, target: &str) -> Result<String, ErrorResult> {
    serde_json::to_string(value).map_err(|error| {
        error!(target, "Error when parsing the struct. {:?}", error);
        internal_server()
    })
}

fn build_response(request: Request, status: u16, json: String) -> Result<Response<Body>, Error> {
    let mut builder = Response::builder();

    if let Some(trace_id) = request
        .headers()
        .get(HEADER_AMZN_TRACE_ID)
        .map(|value| String::from_utf8_lossy(value.as_bytes()).replace("Root=", ""))
    {
        builder = builder.header(HEADER_TRACE_ID, trace_id);
    }

    builder
        .header(CONTENT_TYPE, "application/json")
        .status(status)
        .body(json.into())
        .map(Ok)?
}
