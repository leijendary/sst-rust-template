use database::postgres::connect_postgres;
use lambda::{
    json::json_response, page::ApiPageRequest, query::query_param, tracing::enable_tracing,
};
use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use model::page::PageRequest;
use sample::{repository::SampleRepository, service::SampleService};
use storage::secret::secret_client;

async fn handler(service: &SampleService, event: Request) -> Result<Response<Body>, Error> {
    let query = query_param(&event, "query");
    let page_request = PageRequest::read(&event);
    let result = service.page(&query, &page_request).await;

    json_response(200, result)
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
