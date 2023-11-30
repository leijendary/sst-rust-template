use serde::Serialize;
use serde_with::skip_serializing_none;

pub enum MetaValue {
    Str(String),
    Int(isize),
}

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
}

pub fn internal_server() -> ErrorResult {
    let error = ErrorDetail {
        id: None,
        code: "server_internal".to_string(),
        source: ErrorSource {
            pointer: Some("/server".to_string()),
            header: None,
            parameter: None,
        },
    };

    ErrorResult {
        status: 500,
        errors: vec![error],
    }
}
