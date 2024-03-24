use database::postgres::connect_postgres;
use lambda::{
    header::get_language,
    json::{error_response, json_response},
    path::path_param,
    tracing::enable_tracing,
};
use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sample::{repository::SampleRepository, service::SampleService};
use storage::secret::secret_client;

async fn handler(service: &SampleService, request: Request) -> Result<Response<Body>, Error> {
    let id = match path_param::<i64>(&request, "id") {
        Ok(id) => id,
        Err(error) => return error_response(error),
    };
    let language = get_language(&request);
    let result = service.get(id, false, &language).await;

    json_response(200, result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_tracing();

    let secret_client = secret_client().await;
    let db = connect_postgres(&secret_client).await;
    let repository = SampleRepository { db };
    let service = &SampleService { repository };

    run(service_fn(|request| handler(service, request))).await
}
