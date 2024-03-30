use serde::{ser::Error, Serializer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn serialize_option_offset_date_time<S: Serializer>(
    date_time: &Option<OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match date_time {
        Some(date_time) => date_time
            .format(&Rfc3339)
            .map(|fmt| serializer.serialize_str(&fmt))
            .map_err(Error::custom)?,
        None => serializer.serialize_none(),
    }
}
