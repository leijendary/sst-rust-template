use serde::Serialize;

pub enum MetaValue {
    Str(String),
    Int(isize),
}

#[derive(Serialize)]
pub struct ErrorResult {
    pub status: u16,
    pub errors: Vec<ErrorDetail>,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub code: String,
    pub source: ErrorSource,
}

#[derive(Serialize)]
pub struct ErrorSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
