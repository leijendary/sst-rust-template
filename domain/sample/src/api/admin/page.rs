use lambda::{
    json::json_response, page::ApiPageRequest, query::query_param, tracing::init_tracing,
};
use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use model::page::PageRequest;
use sample::service::SampleService;

async fn handler(service: &SampleService, request: Request) -> Result<Response<Body>, Error> {
    let query = query_param(&request, "query");
    let page_request = PageRequest::read(&request);
    let result = service.page(&query, &page_request).await;

    json_response(200, result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let service = &SampleService::default().await;

    run(service_fn(|request| handler(service, request))).await
}
