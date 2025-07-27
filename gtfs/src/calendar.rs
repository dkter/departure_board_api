use serde::Deserialize;

/// Deserialize GTFS Date-type strings, which are in the format YYYYMMDD
pub fn deserialize_date<'de, D>(deserializer: D) -> Result<chrono::NaiveDate, D::Error> where D: serde::de::Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    let dt = chrono::NaiveDate::parse_from_str(&s, "%Y%m%d").map_err(serde::de::Error::custom)?;
    Ok(dt)
}

/// GTFS Calendar struct that supports deserializing from calendar.txt via serde and csv.
#[derive(Debug, serde::Deserialize)]
pub struct Calendar {
    #[serde(skip_deserializing)]
    pub agency: String,
    pub service_id: String,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: chrono::NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_date: chrono::NaiveDate,
}