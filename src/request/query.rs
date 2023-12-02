use std::str::FromStr;

use lambda_http::{Request, RequestExt};

use crate::error::result::{required_parameter, ErrorResult};

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

pub fn query_version(request: &Request) -> Result<i16, ErrorResult> {
    match query_param(request, "version") {
        Some(version) => Ok(version),
        None => Err(required_parameter("version")),
    }
}
