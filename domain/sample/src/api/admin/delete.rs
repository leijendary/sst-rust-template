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

async fn handler(service: &SampleService, request: Request) -> Result<Response<Body>, Error> {
    let user_id = match get_user_id(&request) {
        Ok(user_id) => user_id,
        Err(error) => return error_response(request, error),
    };
    let id = match path_param::<i64>(&request, "id") {
        Ok(id) => id,
        Err(error) => return error_response(request, error),
    };
    let version = query_version(&request);
    let result = service.delete(id, version, user_id).await;

    json_response(request, 204, result)
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
