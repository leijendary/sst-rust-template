use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
};

use crate::{
    error::result::{ErrorDetail, ErrorResult, ErrorSource},
    model::translation::Translation,
};
use serde_json::Value;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

pub fn validate<T>(value: T) -> Result<T, ErrorResult>
where
    T: Validate,
{
    value.validate().map_err(|errors| ErrorResult {
        status: 400,
        errors: map_validation_error(&errors),
    })?;

    Ok(value)
}

pub fn validate_unique_translation<T>(value: &Vec<T>) -> Result<(), ValidationError>
where
    T: Translation,
{
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

        match key {
            Some(key) => {
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
            None => (),
        }
    }

    Ok(())
}

fn map_validation_error(errors: &ValidationErrors) -> Vec<ErrorDetail> {
    println!("Errors: {:?}", errors);

    errors
        .errors()
        .into_iter()
        .flat_map(|(field, error)| match error {
            ValidationErrorsKind::Struct(errors) => map_struct_errors(errors),
            ValidationErrorsKind::List(errors) => map_list_errors(field, errors),
            ValidationErrorsKind::Field(errors) => map_field_errors(field, errors),
        })
        .collect()
}

fn map_struct_errors(errors: &Box<ValidationErrors>) -> Vec<ErrorDetail> {
    errors
        .field_errors()
        .into_iter()
        .flat_map(|(field, error)| map_field_errors(field, error))
        .collect()
}

fn map_list_errors(field: &str, map: &BTreeMap<usize, Box<ValidationErrors>>) -> Vec<ErrorDetail> {
    map.into_iter()
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

fn map_field_errors(field: &str, errors: &Vec<ValidationError>) -> Vec<ErrorDetail> {
    errors
        .into_iter()
        .map(|err| {
            let mut params = err.params.clone();
            params.remove("value");

            let meta = if !params.is_empty() {
                Some(params)
            } else {
                None
            };

            ErrorDetail {
                id: None,
                code: err.code.to_string(),
                source: ErrorSource {
                    pointer: Some(format!("/body/{field}")),
                    header: None,
                    parameter: None,
                    meta,
                },
            }
        })
        .collect()
}
