use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sst_rust::{
    database::{postgres::connect_postgres, repository::PostgresRepository},
    model::sample::service::SampleService,
    request::{page::Pageable, query::query_param},
    response::json::json_response,
    storage::secret::secret_client,
};

async fn handler(service: &SampleService, event: Request) -> Result<Response<Body>, Error> {
    let query = query_param(&event, "query", "".to_string());
    let pageable = Pageable::new(event);
    let result = service.list(query, pageable).await;

    json_response(200, result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let client = secret_client().await;
    let pool = connect_postgres(&client).await;
    let repository = PostgresRepository::new(pool);
    let service = &SampleService { repository };

    run(service_fn(move |event: Request| async move {
        handler(&service, event).await
    }))
    .await
}
