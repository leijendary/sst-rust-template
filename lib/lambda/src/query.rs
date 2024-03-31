use std::str::FromStr;

use lambda_http::{Request, RequestExt};

pub fn query_param<T: FromStr>(request: &Request, key: &str) -> Option<T> {
    request
        .query_string_parameters_ref()
        .and_then(|query| query.first(key)?.parse::<T>().ok())
}

pub fn query_version(request: &Request) -> i16 {
    query_param(request, "version").unwrap_or_default()
}
