use lambda::{json::json_handler, page::ApiPageRequest, query::query_param, tracing::init_tracing};
use lambda_http::{run, Error, Request};
use lambda_runtime::service_fn;
use model::{
    error::ErrorResult,
    page::{Page, PageRequest},
};
use sample::{model::SampleList, service::SampleService};

async fn handler(
    service: &SampleService,
    request: Request,
) -> Result<(u16, Page<SampleList>), ErrorResult> {
    let query = query_param(&request, "query");
    let page_request = PageRequest::read(&request);
    let result = service.page(&query, &page_request).await?;

    Ok((200, result))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let service = &SampleService::default().await;

    run(service_fn(|request| {
        json_handler(handler(service, request))
    }))
    .await
}
