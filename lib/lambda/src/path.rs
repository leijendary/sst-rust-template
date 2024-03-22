use std::str::FromStr;

use lambda_http::{Request, RequestExt};
use model::error::{required_parameter, ErrorResult};

pub fn path_param<T: FromStr>(request: &Request, key: &str) -> Result<T, ErrorResult> {
    let value = request
        .path_parameters_ref()
        .and_then(|param| param.first(key));

    match value {
        Some(value) => match value.parse::<T>() {
            Ok(value) => Ok(value),
            Err(_) => Err(required_parameter(key)),
        },
        None => Err(required_parameter(key)),
    }
}
