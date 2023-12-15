use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sst_rust::{
    config::tracing::enable_tracing, error::result::path_not_found, response::json::error_response,
};

async fn handler(_: Request) -> Result<Response<Body>, Error> {
    let result = path_not_found();

    error_response(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_tracing();

    run(service_fn(handler)).await
}
