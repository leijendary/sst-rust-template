use database::postgres::connect_postgres;
use lambda::{
    context::get_user_id,
    json::{error_response, json_response},
    tracing::enable_tracing,
};
use lambda_http::{run, Body, Error, Request, RequestPayloadExt, Response};
use lambda_runtime::service_fn;
use model::{error::required_body, validation::validate};
use sample::{model::SampleRequest, repository::SampleRepository, service::SampleService};
use storage::secret::secret_client;

async fn handler(service: &SampleService, request: Request) -> Result<Response<Body>, Error> {
    let user_id = match get_user_id(&request) {
        Ok(user_id) => user_id,
        Err(error) => return error_response(error),
    };
    let sample = match request.payload::<SampleRequest>()? {
        Some(value) => value,
        None => return error_response(required_body()),
    };

    match validate(&sample) {
        Ok(_) => (),
        Err(error) => return error_response(error),
    }

    let result = service.create(sample, user_id).await;

    json_response(201, result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_tracing();

    let client = secret_client().await;
    let pool = connect_postgres(&client).await;
    let repository = SampleRepository { pool };
    let service = &SampleService { repository };

    run(service_fn(|request| handler(service, request))).await
}
