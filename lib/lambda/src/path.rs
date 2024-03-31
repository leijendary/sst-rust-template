use std::str::FromStr;

use lambda_http::{Request, RequestExt};
use model::error::{required_parameter, ErrorResult};

pub fn path_param<T: FromStr>(request: &Request, key: &str) -> Result<T, ErrorResult> {
    request
        .path_parameters_ref()
        .and_then(|param| param.first(key)?.parse::<T>().ok())
        .ok_or_else(|| required_parameter(key))
}
