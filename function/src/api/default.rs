use lambda::{json::json_handler, tracing::init_tracing};
use lambda_http::{run, Error, Request};
use lambda_runtime::service_fn;
use model::error::{path_not_found, ErrorResult};

async fn handler(_: Request) -> Result<(u16, ()), ErrorResult> {
    Err(path_not_found())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    run(service_fn(|request| json_handler(handler(request)))).await
}
