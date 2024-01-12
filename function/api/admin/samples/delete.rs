use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sst_rust_template::{
    config::tracing::enable_tracing,
    database::postgres::connect_postgres,
    domain::sample::{repository::SampleRepository, service::SampleService},
    request::{context::get_user_id, path::path_param, query::query_version},
    response::json::{error_response, json_response},
    storage::secret::secret_client,
};

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
