use lambda_http::{http::header::ACCEPT_LANGUAGE, Request};

pub fn get_accept_language(request: &Request) -> Option<String> {
    match request.headers().get(ACCEPT_LANGUAGE) {
        Some(value) => match value.to_str() {
            Ok(value) => Some(value.to_string()),
            Err(_) => None,
        },
        None => None,
    }
}
