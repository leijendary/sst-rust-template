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
    let sample = request.validate_payload::<SampleRequest>()?;
    let result = service.create(sample, user_id).await?;

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
