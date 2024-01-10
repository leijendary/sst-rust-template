use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use sst_rust_template::{
    config::tracing::enable_tracing,
    database::postgres::connect_postgres,
    domain::sample::{
        model::SampleSeekFilter, repository::SampleRepository, service::SampleService,
    },
    request::{header::get_language, query::query_param, seek::SeekRequest},
    response::json::json_response,
    storage::secret::secret_client,
};

async fn handler(service: &SampleService, event: Request) -> Result<Response<Body>, Error> {
    let language = get_language(&event);
    let query = query_param(&event, "query");
    let filter = &SampleSeekFilter { language, query };
    let seek_request = &SeekRequest::new(&event);
    let result = service.seek(filter, seek_request).await;

    json_response(200, result)
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
