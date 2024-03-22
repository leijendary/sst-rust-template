use database::postgres::connect_postgres;
use lambda::{
    context::get_user_id,
    json::{error_response, json_response},
    path::path_param,
    query::query_version,
    tracing::enable_tracing,
};
use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sample::{repository::SampleRepository, service::SampleService};
use storage::secret::secret_client;

async fn handler(service: &SampleService, event: Request) -> Result<Response<Body>, Error> {
    let user_id = match get_user_id(&event) {
        Ok(user_id) => user_id,
        Err(error) => return error_response(error),
    };
    let id = match path_param::<i64>(&event, "id") {
        Ok(id) => id,
        Err(error) => return error_response(error),
    };
    let version = query_version(&event);
    let result = service.delete(id, version, user_id).await;

    json_response(204, result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_tracing();

    let client = secret_client().await;
    let pool = connect_postgres(&client).await;
    let repository = SampleRepository { pool };
    let service = &SampleService { repository };

    run(service_fn(move |event: Request| async move {
        handler(service, event).await
    }))
    .await
}