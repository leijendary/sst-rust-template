use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
};

use crate::error::{ErrorDetail, ErrorResult, ErrorSource};
use crate::translation::Translation;
use serde_json::Value;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

pub fn validate<T: Validate>(value: T) -> Result<T, ErrorResult> {
    value.validate().map_err(|errors| ErrorResult {
        status: 400,
        errors: map_validation_error(&errors),
    })?;

    Ok(value)
}

pub fn validate_unique_translation<T: Translation>(value: &[T]) -> Result<(), ValidationError> {
    let mut languages: HashSet<String> = HashSet::new();
    let mut ordinals: HashSet<i16> = HashSet::new();

    for (i, value) in value.iter().enumerate() {
        let key = if !languages.insert(value.language()) {
            Some("language")
        } else if !ordinals.insert(value.ordinal()) {
            Some("ordinal")
        } else {
            None
        };

        if key.is_none() {
            continue;
        }

        let error = ValidationError {
            code: Cow::from("duplicate"),
            message: None,
            params: HashMap::from([
                (Cow::from("index"), Value::from(i)),
                (Cow::from("key"), Value::from(key)),
            ]),
        };

        return Err(error);
    }

    Ok(())
}

fn map_validation_error(errors: &ValidationErrors) -> Vec<ErrorDetail> {
    errors
        .errors()
        .iter()
        .flat_map(|(field, error)| match error {
            ValidationErrorsKind::Struct(errors) => map_struct_errors(errors),
            ValidationErrorsKind::List(errors) => map_list_errors(field, errors),
            ValidationErrorsKind::Field(errors) => map_field_errors(field, errors),
        })
        .collect()
}

fn map_struct_errors(errors: &ValidationErrors) -> Vec<ErrorDetail> {
    errors
        .field_errors()
        .into_iter()
        .flat_map(|(field, error)| map_field_errors(field, error))
        .collect()
}

fn map_list_errors(field: &str, map: &BTreeMap<usize, Box<ValidationErrors>>) -> Vec<ErrorDetail> {
    map.iter()
        .flat_map(|(index, errors)| {
            errors
                .field_errors()
                .into_iter()
                .flat_map(move |(key, error)| {
                    let field = format!("{field}/{index}/{key}");
                    map_field_errors(&field, error)
                })
        })
        .collect()
}

fn map_field_errors(field: &str, errors: &[ValidationError]) -> Vec<ErrorDetail> {
    let pointer = format!("/body/{field}");
    errors
        .iter()
        .map(|err| map_error_detail(&pointer, err))
        .collect()
}

fn map_error_detail(pointer: &str, err: &ValidationError) -> ErrorDetail {
    let params: HashMap<String, Value> = err
        .params
        .clone()
        .into_iter()
        .filter(|(key, _)| key != "value")
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    let meta = if !params.is_empty() {
        Some(params)
    } else {
        None
    };

    ErrorDetail {
        id: None,
        code: err.code.to_string(),
        source: ErrorSource {
            pointer: Some(pointer.to_string()),
            header: None,
            parameter: None,
            meta,
        },
    }
}
