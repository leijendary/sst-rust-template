use std::str::FromStr;

use lambda_http::{
    http::header::ACCEPT_LANGUAGE, request::RequestContext, Request, RequestExt, RequestPayloadExt,
};
use model::{
    error::{invalid_body, required_body, required_parameter, unauthorized, ErrorResult},
    validation::validate,
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub trait RequestExtension {
    fn get_user_id(&self) -> Result<String, ErrorResult>;

    fn path_param<T: FromStr>(&self, key: &str) -> Result<T, ErrorResult>;

    fn query_param<T: FromStr>(&self, key: &str) -> Option<T>;

    fn query_version(&self) -> i16;

    fn get_language(&self) -> Option<String>;

    fn validate_payload<P>(&self) -> Result<P, ErrorResult>
    where
        P: DeserializeOwned + Validate;
}

impl RequestExtension for Request {
    fn get_user_id(&self) -> Result<String, ErrorResult> {
        match self.request_context() {
            RequestContext::ApiGatewayV2(http) => http
                .authorizer
                .and_then(|a| a.jwt)
                .map(|j| j.claims)
                .and_then(|c| c.get("sub").map(|s| s.to_owned()))
                .ok_or_else(unauthorized),
            _ => Err(unauthorized()),
        }
    }

    fn path_param<T: FromStr>(&self, key: &str) -> Result<T, ErrorResult> {
        self.path_parameters_ref()
            .and_then(|param| param.first(key)?.parse::<T>().ok())
            .ok_or_else(|| required_parameter(key))
    }

    fn query_param<T: FromStr>(&self, key: &str) -> Option<T> {
        self.query_string_parameters_ref()
            .and_then(|query| query.first(key)?.parse::<T>().ok())
    }

    fn query_version(&self) -> i16 {
        self.query_param("version").unwrap_or_default()
    }

    fn get_language(&self) -> Option<String> {
        self.headers()
            .get(ACCEPT_LANGUAGE)
            .and_then(|value| value.to_str().map(|s| s.to_string()).ok())
    }

    fn validate_payload<P>(&self) -> Result<P, ErrorResult>
    where
        P: DeserializeOwned + Validate,
    {
        match self.payload::<P>() {
            Ok(value) => value.ok_or_else(required_body).and_then(validate),
            Err(_) => Err(invalid_body()),
        }
    }
}
