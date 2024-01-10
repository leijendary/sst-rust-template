use lambda_http::{run, Body, Error, Request, RequestPayloadExt, Response};
use lambda_runtime::service_fn;
use sst_rust_template::{
    config::tracing::enable_tracing,
    database::postgres::connect_postgres,
    domain::sample::{model::SampleRequest, repository::SampleRepository, service::SampleService},
    error::result::required_body,
    request::{context::get_user_id, validator::validate},
    response::json::{error_response, json_response},
    storage::secret::secret_client,
};

async fn handler(service: &SampleService, event: Request) -> Result<Response<Body>, Error> {
    let user_id = match get_user_id(&event) {
        Ok(user_id) => user_id,
        Err(error) => return error_response(error),
    };
    let mut sample = match event.payload::<SampleRequest>()? {
        Some(value) => value,
        None => return error_response(required_body()),
    };

    match validate(&sample) {
        Ok(_) => (),
        Err(error) => return error_response(error),
    }

    sample.created_by = user_id.to_owned();
    sample.last_modified_by = user_id.to_owned();

    let result = service.create(sample).await;

    json_response(201, result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_tracing();

    let client = secret_client().await;
    let pool = connect_postgres(&client, 1).await;
    let repository = SampleRepository { pool };
    let service = &SampleService { repository };

    run(service_fn(move |event: Request| async move {
        handler(service, event).await
    }))
    .await
}
