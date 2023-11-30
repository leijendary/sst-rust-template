use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sst_rust::{
    config::tracing::enable_tracing,
    database::postgres::{connect_postgres, PostgresRepository},
    model::sample::service::SampleService,
    request::{page::PageRequest, query::query_param},
    response::json::json_response,
    storage::secret::secret_client,
};

async fn handler(service: &SampleService, event: Request) -> Result<Response<Body>, Error> {
    let query = query_param(&event, "query");
    let page_request = PageRequest::new(&event);
    let result = service.list(&query, &page_request).await;

    json_response(200, result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_tracing();

    let client = secret_client().await;
    let pool = connect_postgres(&client).await;
    let repository = PostgresRepository::new(pool);
    let service = &SampleService { repository };

    run(service_fn(move |event: Request| async move {
        handler(&service, event).await
    }))
    .await
}