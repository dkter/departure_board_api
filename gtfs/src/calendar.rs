use serde::Deserialize;

/// Deserialize GTFS Date-type strings, which are in the format YYYYMMDD
pub fn deserialize_date<'de, D>(deserializer: D) -> Result<chrono::NaiveDate, D::Error> where D: serde::de::Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    let dt = chrono::NaiveDate::parse_from_str(&s, "%Y%m%d").map_err(serde::de::Error::custom)?;
    Ok(dt)
}

/// Deserialize "0" or "1" into booleans
pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: serde::de::Deserializer<'de> {
    match String::deserialize(deserializer)?.as_ref() {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(other),
            &"0 or 1",
        )),
    }
}

/// GTFS Calendar struct that supports deserializing from calendar.txt via serde and csv.
#[derive(Debug, serde::Deserialize)]
pub struct Calendar {
    #[serde(skip_deserializing)]
    pub agency: String,
    pub service_id: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub monday: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub tuesday: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wednesday: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub thursday: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub friday: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub saturday: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub sunday: bool,
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: chrono::NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_date: chrono::NaiveDate,
}