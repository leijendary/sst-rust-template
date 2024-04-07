use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;
use serde_with::skip_serializing_none;

#[derive(Debug, Serialize)]
pub struct ErrorResult {
    pub status: u16,
    pub errors: Vec<ErrorDetail>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub id: Option<Value>,
    pub code: String,
    pub source: ErrorSource,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct ErrorSource {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
    pub header: Option<String>,
    pub meta: Option<HashMap<String, Value>>,
}

pub fn internal_server() -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: "server_internal".to_owned(),
        source: ErrorSource {
            pointer: Some("/server".to_owned()),
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
        code: "unauthorized".to_owned(),
        source: ErrorSource {
            pointer: None,
            header: Some("authorization".to_owned()),
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
        code: "required".to_owned(),
        source: ErrorSource {
            pointer: Some("/body".to_owned()),
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
        code: "required".to_owned(),
        source: ErrorSource {
            pointer: None,
            header: None,
            parameter: Some(name.to_owned()),
            meta: None,
        },
    };

    ErrorResult {
        status: 400,
        errors: vec![error],
    }
}

pub fn invalid_parameter(name: String) -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: "invalid".to_owned(),
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

pub fn id_not_found(entity: &str, id: i64) -> ErrorResult {
    let pointer = format!("/data/{entity}/id");
    let error = ErrorDetail {
        id: Some(Value::from(id)),
        code: "not_found".to_owned(),
        source: ErrorSource {
            pointer: Some(pointer),
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

pub fn version_conflict(entity: &str, id: i64, version: i16) -> ErrorResult {
    let pointer = format!("/data/{entity}/version");
    let meta = HashMap::from([("version".to_owned(), Value::from(version))]);
    let error = ErrorDetail {
        id: Some(Value::from(id)),
        code: "version_conflict".to_owned(),
        source: ErrorSource {
            pointer: Some(pointer),
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

pub fn path_not_found() -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: "not_found".to_owned(),
        source: ErrorSource {
            pointer: Some("/path".to_owned()),
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

pub fn invalid_body() -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: "invalid".to_owned(),
        source: ErrorSource {
            pointer: Some("/body".to_owned()),
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
