use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;
use serde_json::Value;
use serde_with::skip_serializing_none;

#[derive(Serialize)]
pub struct ErrorResult {
    pub status: u16,
    pub errors: Vec<ErrorDetail>,
}

#[skip_serializing_none]
#[derive(Serialize)]
pub struct ErrorDetail {
    pub id: Option<String>,
    pub code: String,
    pub source: ErrorSource,
}

#[skip_serializing_none]
#[derive(Serialize)]
pub struct ErrorSource {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
    pub header: Option<String>,
    pub meta: Option<HashMap<Cow<'static, str>, Value>>,
}

pub fn internal_server() -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: "server_internal".to_string(),
        source: ErrorSource {
            pointer: Some("/server".to_string()),
            header: None,
            parameter: None,
            meta: None,
        },
    };

    ErrorResult {
        status: 500,
        errors: vec![error],
    }
}

pub fn required_body() -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: "required".to_string(),
        source: ErrorSource {
            pointer: Some("/body".to_string()),
            header: None,
            parameter: None,
            meta: None,
        },
    };

    ErrorResult {
        status: 400,
        errors: vec![error],
    }
}

pub fn required_parameter(name: &str) -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: "required".to_string(),
        source: ErrorSource {
            pointer: None,
            header: None,
            parameter: Some(name.to_string()),
            meta: None,
        },
    };

    ErrorResult {
        status: 400,
        errors: vec![error],
    }
}

pub fn invalid_parameter(name: &str) -> ErrorResult {
    let error: ErrorDetail = ErrorDetail {
        id: None,
        code: "invalid".to_string(),
        source: ErrorSource {
            pointer: None,
            header: None,
            parameter: Some(name.to_string()),
            meta: None,
        },
    };

    ErrorResult {
        status: 400,
        errors: vec![error],
    }
}
