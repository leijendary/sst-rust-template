use super::result::{ErrorDetail, ErrorResult};
use crate::error::result::{internal_server, ErrorSource};
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::{
    error::{
        DatabaseError,
        ErrorKind::{
            CheckViolation, ForeignKeyViolation, NotNullViolation, Other, UniqueViolation,
        },
    },
    postgres::PgDatabaseError,
    Error,
};
use std::vec;
use tracing::error;

lazy_static! {
    static ref UNIQUE_REGEX: Regex =
        Regex::new(r"Key \((?:lower\()?([a-zA-Z0-9, ]+)+(?:::text)?\)").unwrap();
}

pub fn resource_error(id: i64, pointer: &str, err: Error) -> ErrorResult {
    if !matches!(err, Error::RowNotFound) {
        return database_error(err);
    }

    let error = ErrorDetail {
        id: Some(id.to_string()),
        code: "not_found".to_string(),
        source: ErrorSource {
            pointer: Some(pointer.to_owned()),
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

pub fn database_error(err: Error) -> ErrorResult {
    let (status, detail) = match err.as_database_error() {
        Some(err) => parse_detail(err.downcast_ref()),
        None => {
            error!(target: "database_error", "Something failed in the database. {}", err.to_string());
            return internal_server();
        }
    };

    ErrorResult {
        status,
        errors: vec![detail],
    }
}

fn parse_detail(err: &PgDatabaseError) -> (u16, ErrorDetail) {
    error!(target: "database_error", "Failed to execute a database query {:?}", err);

    let (status, code, pointer) = match err.kind() {
        UniqueViolation => {
            let table = err.table().unwrap_or("").to_string();
            let field = UNIQUE_REGEX
                .captures(err.detail().unwrap())
                .map(|m| m.get(1).unwrap().as_str())
                .unwrap()
                .split(", ")
                .last()
                .unwrap()
                .to_case(Case::Camel);
            let pointer = format!("/data/{table}/{field}");

            (409, "duplicate".to_string(), pointer)
        }
        ForeignKeyViolation => (404, "not_found".to_string(), "".to_string()),
        NotNullViolation => (400, "required".to_string(), "".to_string()),
        CheckViolation => (400, "invalid".to_string(), "".to_string()),
        Other | _ => (500, "server_internal".to_string(), "/server".to_string()),
    };
    let detail = ErrorDetail {
        id: None,
        code,
        source: ErrorSource {
            pointer: Some(pointer),
            parameter: None,
            header: None,
            meta: None,
        },
    };

    (status, detail)
}
