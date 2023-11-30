use std::str::FromStr;

use lambda_http::{Request, RequestExt};

pub fn query_param<T: FromStr>(request: &Request, key: &str) -> Option<T> {
    let value = request
        .query_string_parameters_ref()
        .and_then(|query| query.first(key));

    match value {
        Some(value) => match value.parse::<T>() {
            Ok(value) => Some(value),
            Err(_) => None,
        },
        None => None,
    }
}
