use lambda::{
    header::get_language, json::json_response, query::query_param, seek::ApiSeekRequest,
    tracing::init_tracing,
};
use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use model::seek::SeekRequest;
use sample::{model::SampleSeekFilter, service::SampleService};

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
    init_tracing();

    let service = &SampleService::default().await;

    run(service_fn(|request| handler(service, request))).await
}
