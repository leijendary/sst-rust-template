use serde::Serializer;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn serialize_option_offset_date_time<S>(
    date_time: &Option<OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date_time {
        Some(date_time) => {
            let formatted = date_time.format(&Rfc3339).unwrap();
            serializer.serialize_str(&formatted)
        }
        None => serializer.serialize_none(),
    }
}
