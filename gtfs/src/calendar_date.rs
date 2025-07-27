use crate::calendar::deserialize_date;

/// GTFS CalendarDate struct that supports deserializing from calendar_dates.txt via serde and csv.
#[derive(Debug, serde::Deserialize)]
pub struct CalendarDate {
    #[serde(skip_deserializing)]
    pub agency: String,
    pub service_id: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: chrono::NaiveDate,
    pub exception_type: i32,
}