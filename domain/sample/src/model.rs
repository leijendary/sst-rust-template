use model::seek::Seekable;
use model::{translation::Translation, validation::validate_unique_translation};
use serde::{Deserialize, Serialize};
use serde_trim::{option_string_trim, string_trim};
use serde_with::skip_serializing_none;
use sqlx::prelude::FromRow;
use time::{serde::rfc3339, OffsetDateTime};
use validator::Validate;

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
    pub amount: i32,
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
    pub description: Option<String>,
    #[serde(default)]
    // Last 2 digits are decimals
    #[validate(range(min = 1, max = 99999999))]
    pub amount: i32,
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
    #[validate(range(min = 1, max = 100))]
    pub ordinal: i16,
}

impl Translation for SampleTranslation {
    fn language(&self) -> String {
        self.language.to_owned()
    }

    fn ordinal(&self) -> i16 {
        self.ordinal.to_owned()
    }
}

#[skip_serializing_none]
#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleDetail {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub amount: i32,
    pub version: i16,
    #[sqlx(skip)]
    pub translations: Option<Vec<SampleTranslation>>,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}
