use lambda::{json::error_response, tracing::enable_tracing};
use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use model::error::path_not_found;

async fn handler(_: Request) -> Result<Response<Body>, Error> {
    let result = path_not_found();

    error_response(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_tracing();

    run(service_fn(handler)).await
}
