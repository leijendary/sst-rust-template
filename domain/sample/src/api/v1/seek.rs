use database::postgres::connect_postgres;
use lambda::{
    header::get_language, json::json_response, query::query_param, seek::ApiSeekRequest,
    tracing::enable_tracing,
};
use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use model::seek::SeekRequest;
use sample::{model::SampleSeekFilter, repository::SampleRepository, service::SampleService};
use storage::secret::secret_client;

async fn handler(service: &SampleService, request: Request) -> Result<Response<Body>, Error> {
    let language = get_language(&request);
    let query = query_param(&request, "query");
    let filter = &SampleSeekFilter { language, query };
    let seek_request = &SeekRequest::read(&request);
    let result = service.seek(filter, seek_request).await;

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
