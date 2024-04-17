use model::seek::Seekable;
use model::{
    translation::Translation, validation::validate_decimal_range,
    validation::validate_unique_translation,
};
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_trim::{option_string_trim, string_trim};
use serde_with::skip_serializing_none;
use sqlx::prelude::FromRow;
use sqlx::types::Decimal;
use time::{serde::rfc3339, OffsetDateTime};
use validator::{Validate, ValidationError};

pub struct SampleSeekFilter {
    pub language: Option<String>,
    pub query: Option<String>,
}

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleList {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: Decimal,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

impl Seekable for SampleList {
    fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    fn id(&self) -> i64 {
        self.id
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct SampleRequest {
    #[serde(default, deserialize_with = "string_trim")]
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(default, deserialize_with = "option_string_trim")]
    #[validate(length(max = 2000))]
    pub description: Option<String>,
    #[serde(default)]
    #[validate(custom(function = "validate_amount"))]
    pub amount: Decimal,
    #[serde(default)]
    #[validate(
        length(min = 1, max = 100),
        custom(function = "validate_unique_translation")
    )]
    #[validate]
    pub translations: Vec<SampleTranslation>,
}

#[derive(Debug, FromRow, Validate, Serialize, Deserialize)]
pub struct SampleTranslation {
    #[serde(default, deserialize_with = "string_trim")]
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "option_string_trim"
    )]
    #[validate(length(max = 200))]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "string_trim")]
    #[validate(length(min = 2, max = 4))]
    pub language: String,
    #[serde(default)]
    #[validate(range(min = 1, max = 100))]
    pub ordinal: i16,
}

impl Translation for SampleTranslation {
    fn language(&self) -> String {
        self.language.to_owned()
    }

    fn ordinal(&self) -> i16 {
        self.ordinal
    }
}

#[skip_serializing_none]
#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleDetail {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub amount: Decimal,
    pub version: i16,
    #[sqlx(skip)]
    pub translations: Option<Vec<SampleTranslation>>,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

pub struct SampleTranslationsBinds {
    pub names: Vec<String>,
    pub descriptions: Vec<Option<String>>,
    pub languages: Vec<String>,
    pub ordinals: Vec<i16>,
}

impl From<Vec<SampleTranslation>> for SampleTranslationsBinds {
    fn from(val: Vec<SampleTranslation>) -> Self {
        let len = val.len();
        let mut names = Vec::<String>::with_capacity(len);
        let mut descriptions = Vec::<Option<String>>::with_capacity(len);
        let mut languages = Vec::<String>::with_capacity(len);
        let mut ordinals = Vec::<i16>::with_capacity(len);

        for translation in val {
            names.push(translation.name);
            descriptions.push(translation.description);
            languages.push(translation.language);
            ordinals.push(translation.ordinal);
        }

        SampleTranslationsBinds {
            names,
            descriptions,
            languages,
            ordinals,
        }
    }
}

fn validate_amount(amount: &Decimal) -> Result<(), ValidationError> {
    static MIN: Decimal = dec!(0.01);
    static MAX: Decimal = dec!(999999999.99);

    validate_decimal_range(amount, MIN, MAX)
}
