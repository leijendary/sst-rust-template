use lambda::{
    context::get_user_id, json::json_handler, path::path_param, query::query_version,
    tracing::init_tracing,
};
use lambda_http::{run, Error, Request};
use lambda_runtime::service_fn;
use model::error::ErrorResult;
use sample::service::SampleService;

async fn handler(service: &SampleService, request: Request) -> Result<(u16, ()), ErrorResult> {
    let user_id = get_user_id(&request)?;
    let id = path_param::<i64>(&request, "id")?;
    let version = query_version(&request);

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
