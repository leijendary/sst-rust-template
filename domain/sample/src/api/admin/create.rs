use lambda::{
    context::get_user_id, json::json_handler, request::RequestPayloadParser, tracing::init_tracing,
};
use lambda_http::{run, Error, Request};
use lambda_runtime::service_fn;
use model::{error::ErrorResult, validation::validate};
use sample::{
    model::{SampleDetail, SampleRequest},
    service::SampleService,
};

async fn handler(
    service: &SampleService,
    request: Request,
) -> Result<(u16, SampleDetail), ErrorResult> {
    let user_id = get_user_id(&request)?;
    let sample = request
        .parse_payload::<SampleRequest>()
        .and_then(validate)?;
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
