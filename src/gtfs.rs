use std::{collections::BTreeMap, io::{Read, Seek}};

pub trait FromZip {
    fn from_zip<R: Read + Seek>(zip: &mut zip::ZipArchive<R>, agency: &str) -> Self;
}

#[derive(Debug, serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, PartialEq)]
pub struct Stop {
    #[serde(skip_deserializing)]
    pub agency: String,
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: String,
    pub tts_stop_name: Option<String>,
    pub stop_desc: Option<String>,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    pub location_type: Option<String>,
    pub parent_station: Option<String>,
    pub stop_timezone: Option<String>,
    pub wheelchair_boarding: Option<u32>,
    pub level_id: Option<String>,
    pub platform_code: Option<String>,
}

impl FromZip for Vec<Stop> {
    fn from_zip<R: Read + Seek>(zip: &mut zip::ZipArchive<R>, agency: &str) -> Self {
        let file = zip.by_name("stops.txt")
            .expect("zip did not have stops.txt");
        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::Fields)
            .flexible(true)
            .from_reader(file);
        let mut stops = Vec::new();
        for result in reader.deserialize() {
            let mut stop: Stop = result
                .expect("could not deserialize stop");
            stop.agency = String::from(agency);
            stops.push(stop);
        }
        stops
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GtfsTime(u8, u8, u8);

impl GtfsTime {
    pub fn from_chrono_time(ch: impl chrono::Timelike) -> Self {
        Self(ch.hour() as u8, ch.minute() as u8, ch.second() as u8)
    }

    pub fn local_now() -> Self {
        let now = chrono::Local::now();
        Self::from_chrono_time(now)
    }
}

mod gtfs_time_serde {
    use super::GtfsTime;
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        date: &GtfsTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}:{}:{}", date.0, date.1, date.2);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<GtfsTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut split = s.split(':');
        let h = split.next()
            .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
            .parse::<u8>().map_err(serde::de::Error::custom)?;
        let m = split.next()
            .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
            .parse::<u8>().map_err(serde::de::Error::custom)?;
        let s = split.next()
            .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
            .parse::<u8>().map_err(serde::de::Error::custom)?;
        Ok(GtfsTime(h, m, s))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StopTime {
    #[serde(skip_deserializing)]
    pub agency: String,
    pub trip_id: String,
    #[serde(with = "gtfs_time_serde")]
    pub arrival_time: GtfsTime,
    #[serde(with = "gtfs_time_serde")]
    pub departure_time: GtfsTime,
    pub stop_id: Option<String>,
    pub location_group_id: Option<String>,
    pub location_id: Option<String>,
    pub stop_sequence: i32,
    pub stop_headsign: Option<String>,
    pub start_pickup_drop_off_window: Option<String>,
    pub end_pickup_drop_off_window: Option<String>,
    pub pickup_type: Option<String>,
    pub drop_off_type: Option<String>,
    pub continuous_pickup: Option<String>,
    pub continuous_drop_off: Option<String>,
    pub shape_dist_traveled: f64,
    pub timepoint: Option<String>,
    pub pickup_booking_rule_id: Option<String>,
    pub drop_off_booking_rule_id: Option<String>,
}

impl FromZip for BTreeMap<GtfsTime, StopTime> {
    fn from_zip<R: Read + Seek>(zip: &mut zip::ZipArchive<R>, agency: &str) -> Self {
        let file = zip.by_name("stop_times.txt")
            .expect("zip did not have stop_times.txt");
        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::Fields)
            .flexible(true)
            .from_reader(file);
        let mut stop_times = BTreeMap::new();
        for result in reader.deserialize() {
            let mut stop_time: StopTime = result
                .expect("could not deserialize stop time");
            stop_time.agency = String::from(agency);
            stop_times.insert(stop_time.departure_time, stop_time);
        }
        stop_times
    }
}