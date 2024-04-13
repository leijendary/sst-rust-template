use lambda::{json::json_handler, request::RequestExtension, tracing::init_tracing};
use lambda_http::{run, Error, Request};
use lambda_runtime::service_fn;
use model::error::ErrorResult;
use sample::service::SampleService;

async fn handler(service: &SampleService, request: Request) -> Result<(u16, ()), ErrorResult> {
    let user_id = request.get_user_id()?;
    let id = request.path_param::<i64>("id")?;
    let version = request.query_version();

    service.delete(id, version, user_id).await?;

    Ok((204, ()))
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
