use lambda::{
    header::get_language,
    json::{error_response, json_response},
    path::path_param,
    tracing::init_tracing,
};
use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sample::service::SampleService;

async fn handler(service: &SampleService, request: Request) -> Result<Response<Body>, Error> {
    let id = match path_param::<i64>(&request, "id") {
        Ok(id) => id,
        Err(error) => return error_response(error),
    };
    let language = get_language(&request);
    let result = service.get(id, false, &language).await;

    json_response(200, result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let service = &SampleService::default().await;

    run(service_fn(|request| handler(service, request))).await
}
