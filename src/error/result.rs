use std::{borrow::Cow, collections::HashMap};

use lambda_http::http::header::AUTHORIZATION;
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
    pub id: Option<Cow<'static, str>>,
    pub code: Cow<'static, str>,
    pub source: ErrorSource,
}

#[skip_serializing_none]
#[derive(Serialize)]
pub struct ErrorSource {
    pub pointer: Option<Cow<'static, str>>,
    pub parameter: Option<Cow<'static, str>>,
    pub header: Option<Cow<'static, str>>,
    pub meta: Option<HashMap<Cow<'static, str>, Value>>,
}

pub fn internal_server() -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: Cow::from("server_internal"),
        source: ErrorSource {
            pointer: Some(Cow::from("/server")),
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

pub fn unauthorized() -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: Cow::from("unauthorized"),
        source: ErrorSource {
            pointer: None,
            header: Some(Cow::from(AUTHORIZATION.as_str())),
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
        code: Cow::from("required"),
        source: ErrorSource {
            pointer: Some(Cow::from("/body")),
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

pub fn required_parameter(name: &'static str) -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: Cow::from("required"),
        source: ErrorSource {
            pointer: None,
            header: None,
            parameter: Some(Cow::from(name)),
            meta: None,
        },
    };

    ErrorResult {
        status: 400,
        errors: vec![error],
    }
}

pub fn invalid_parameter(name: Cow<'static, str>) -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: Cow::from("invalid"),
        source: ErrorSource {
            pointer: None,
            header: None,
            parameter: Some(name),
            meta: None,
        },
    };

    ErrorResult {
        status: 400,
        errors: vec![error],
    }
}

pub fn resource_not_found(id: i64, pointer: &'static str) -> ErrorResult {
    let error = ErrorDetail {
        id: Some(Cow::from(id.to_string())),
        code: Cow::from("not_found"),
        source: ErrorSource {
            pointer: Some(Cow::from(pointer)),
            parameter: None,
            header: None,
            meta: None,
        },
    };

    ErrorResult {
        status: 404,
        errors: vec![error],
    }
}

pub fn version_conflict(id: i64, pointer: &'static str, version: i16) -> ErrorResult {
    let meta = HashMap::from([(Cow::from("version"), Value::from(version))]);
    let error = ErrorDetail {
        id: Some(Cow::from(id.to_string())),
        code: Cow::from("version_conflict"),
        source: ErrorSource {
            pointer: Some(Cow::from(pointer)),
            parameter: None,
            header: None,
            meta: Some(meta),
        },
    };

    ErrorResult {
        status: 409,
        errors: vec![error],
    }
}

pub fn not_found() -> ErrorResult {
    let error = ErrorDetail {
        code: Cow::from("not_found"),
        source: ErrorSource {
            pointer: Some(Cow::from("/path")),
            parameter: None,
            header: None,
            meta: None,
        },
        id: None,
    };

    ErrorResult {
        status: 404,
        errors: vec![error],
    }
}
