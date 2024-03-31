use lambda_http::{http::header::ACCEPT_LANGUAGE, Request};

pub fn get_language(request: &Request) -> Option<String> {
    request
        .headers()
        .get(ACCEPT_LANGUAGE)
        .and_then(|value| value.to_str().map(|s| s.to_string()).ok())
}
