use lambda::{json::json_handler, request::RequestExtension, tracing::init_tracing};
use lambda_http::{run, Error, Request};
use lambda_runtime::service_fn;
use model::error::ErrorResult;
use sample::{model::SampleDetail, service::SampleService};

async fn handler(
    service: &SampleService,
    request: Request,
) -> Result<(u16, SampleDetail), ErrorResult> {
    let id = request.path_param::<i64>("id")?;
    let language = request.get_language();
    let result = service.get(id, false, &language).await?;

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
