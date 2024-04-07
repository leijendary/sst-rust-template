use lambda::{
    header::get_language, json::json_handler, query::query_param, seek::ApiSeekRequest,
    tracing::init_tracing,
};
use lambda_http::{run, Error, Request};
use lambda_runtime::service_fn;
use model::{
    error::ErrorResult,
    seek::{Seek, SeekRequest},
};
use sample::{
    model::{SampleList, SampleSeekFilter},
    service::SampleService,
};

async fn handler(
    service: &SampleService,
    request: Request,
) -> Result<(u16, Seek<SampleList>), ErrorResult> {
    let language = get_language(&request);
    let query = query_param(&request, "query");
    let filter = &SampleSeekFilter { language, query };
    let seek_request = &SeekRequest::read(&request);
    let result = service.seek(filter, seek_request).await?;

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
