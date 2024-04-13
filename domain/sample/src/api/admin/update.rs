use lambda::{json::json_handler, request::RequestExtension, tracing::init_tracing};
use lambda_http::{run, Error, Request};
use lambda_runtime::service_fn;
use model::error::ErrorResult;
use sample::{
    model::{SampleDetail, SampleRequest},
    service::SampleService,
};

async fn handler(
    service: &SampleService,
    request: Request,
) -> Result<(u16, SampleDetail), ErrorResult> {
    let user_id = request.get_user_id()?;
    let id = request.path_param::<i64>("id")?;
    let sample = request.validate_payload::<SampleRequest>()?;
    let version = request.query_version();
    let result = service.update(id, sample, version, user_id).await?;

    Ok((201, result))
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
