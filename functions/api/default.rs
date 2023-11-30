use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sst_rust::{
    config::tracing::enable_tracing,
    error::result::{ErrorDetail, ErrorResult, ErrorSource},
    response::json::json_response,
};

async fn handler(_: Request) -> Result<Response<Body>, Error> {
    let error = ErrorDetail {
        code: "not_found".to_string(),
        source: ErrorSource {
            pointer: Some("/path".to_string()),
            parameter: None,
            header: None,
        },
        id: None,
    };
    let result = ErrorResult {
        status: 404,
        errors: vec![error],
    };

    json_response(result.status, Ok(result))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_tracing();

    run(service_fn(handler)).await
}
