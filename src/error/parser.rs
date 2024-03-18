use super::result::{id_not_found, version_conflict, ErrorDetail, ErrorResult};
use crate::error::result::{internal_server, ErrorSource};
use convert_case::{Case, Casing};
use once_cell::sync::Lazy;
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

static UNIQUE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Key \((?:lower\()?([a-zA-Z0-9, ]+)+(?:::text)?\)")
        .expect("UNIQUE_REGEX is not a valid pattern")
});

pub fn resource_error(entity: &str, id: i64, version: Option<i16>, err: Error) -> ErrorResult {
    if !matches!(err, Error::RowNotFound) {
        return database_error(err);
    }

    match version {
        Some(version) => version_conflict(entity, id, version),
        None => id_not_found(entity, id),
    }
}

pub fn database_error(err: Error) -> ErrorResult {
    let (status, detail) = match err.as_database_error() {
        Some(err) => parse_detail(err.downcast_ref()),
        None => {
            error!(target: "database_error", "Something failed in the database. {:?}", err);
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
        UniqueViolation => unique_violation(err),
        ForeignKeyViolation => foreign_key_violation(err),
        NotNullViolation => not_null_violation(err),
        CheckViolation => check_violation(err),
        Other => other_violation(err),
        _ => other_violation(err),
    };
    let error = ErrorDetail {
        id: None,
        code,
        source: ErrorSource {
            pointer: Some(pointer),
            parameter: None,
            header: None,
            meta: None,
        },
    };

    (status, error)
}

fn unique_violation(err: &PgDatabaseError) -> (u16, String, String) {
    let status = 409;
    let code = "duplicate".to_owned();
    let mut pointer = "/data".to_owned();
    let table = match err.table() {
        Some(table) => table,
        None => {
            error!(target: "unique_violation", "Unique violation but no table defined. {:?}", err);
            return (status, code, pointer);
        }
    };
    let detail = match err.detail() {
        Some(detail) => detail,
        None => {
            error!(target: "unique_violation", "Unique violation but no detail defined. {:?}", err);
            return (status, code, pointer);
        }
    };
    let matched = UNIQUE_REGEX
        .captures(detail)
        .and_then(|m| m.get(1))
        .and_then(|s| s.as_str().split(", ").last())
        .map(|s| s.to_case(Case::Camel));
    let field = match matched {
        Some(field) => field,
        None => {
            error!(target: "unique_violation", "Unique violation but no field found. {:?}", err);
            return (status, code, pointer);
        }
    };
    pointer = format!("{pointer}/{table}/{field}");

    (status, code, pointer)
}

fn foreign_key_violation(_: &PgDatabaseError) -> (u16, String, String) {
    todo!()
}

fn not_null_violation(_: &PgDatabaseError) -> (u16, String, String) {
    todo!()
}

fn check_violation(_: &PgDatabaseError) -> (u16, String, String) {
    todo!()
}

fn other_violation(_: &PgDatabaseError) -> (u16, String, String) {
    todo!()
}
