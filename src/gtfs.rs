use std::{collections::BTreeMap, io::{Read, Seek}};

pub trait FromZip<R: Read + Seek> {
    fn from_zip(zip: &mut zip::ZipArchive<R>, agency: &str) -> Self;
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

impl<R: Read + Seek> FromZip<R> for Vec<Stop> {
    fn from_zip(zip: &mut zip::ZipArchive<R>, agency: &str) -> Self {
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

impl From<i32> for GtfsTime {
    /// Here, value is a number of seconds since midnight
    fn from(value: i32) -> Self {
        let hrs = value / 3600;
        let mins = (value % 3600) / 60;
        let secs = (value % 3600) % 60;
        Self(hrs as u8, mins as u8, secs as u8)
    }
}

impl Into<i32> for GtfsTime {
    /// Returns the GtfsTime as a number of seconds since midnight
    fn into(self) -> i32 {
        (self.0 as i32) * 3600 + (self.1 as i32) * 60 + (self.2 as i32)
    }
}

impl serde::Serialize for GtfsTime {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}:{}:{}", self.0, self.1, self.2);
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for GtfsTime {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
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
        Ok(Self(h, m, s))
    }
}

// mod gtfs_time_serde {
//     use super::GtfsTime;
//     use serde::{self, Deserialize, Serializer, Deserializer};

//     pub fn serialize<S>(
//         date: &Option<GtfsTime>,
//         serializer: S,
//     ) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         if let Some(date) = date {
//             let s = format!("{}:{}:{}", date.0, date.1, date.2);
//             serializer.serialize_some(&s)
//         } else {
//             serializer.serialize_none()
//         }
//     }

//     pub fn deserialize<'de, D>(
//         deserializer: D,
//     ) -> Result<Option<GtfsTime>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         if s.is_empty() {
//             return Ok(None);
//         }
//         let mut split = s.split(':');
//         let h = split.next()
//             .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
//             .parse::<u8>().map_err(serde::de::Error::custom)?;
//         let m = split.next()
//             .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
//             .parse::<u8>().map_err(serde::de::Error::custom)?;
//         let s = split.next()
//             .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
//             .parse::<u8>().map_err(serde::de::Error::custom)?;
//         Ok(Some(GtfsTime(h, m, s)))
//     }
// }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StopTime {
    #[serde(skip_deserializing)]
    pub agency: String,
    pub trip_id: String,
    pub arrival_time: Option<GtfsTime>,
    pub departure_time: Option<GtfsTime>,
    pub stop_id: Option<String>,
    pub location_group_id: Option<String>,
    pub location_id: Option<String>,
    pub stop_sequence: i32,
    pub stop_headsign: Option<String>,
    pub start_pickup_drop_off_window: Option<GtfsTime>,
    pub end_pickup_drop_off_window: Option<GtfsTime>,
    pub pickup_type: Option<i32>,
    pub drop_off_type: Option<i32>,
    pub continuous_pickup: Option<i32>,
    pub continuous_drop_off: Option<i32>,
    pub shape_dist_traveled: Option<f32>,
    pub timepoint: Option<i32>,
    pub pickup_booking_rule_id: Option<String>,
    pub drop_off_booking_rule_id: Option<String>,
}

// pub struct StopTimeReader<'a, R: Read + Seek> {
//     agency: String,
//     reader: csv::Reader<'a, zip::read::ZipFile<'a, R>>,
//     iter: csv::DeserializeRecordsIter<'a, zip::read::ZipFile<'a, R>, StopTime>
// }

// impl<'a, R: Read + Seek> Iterator for StopTimeReader<'a, R> {
//     type Item = StopTime;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(result) = self.iter.next() {
//             let mut stop_time: StopTime = result
//                 .expect("could not deserialize stop time");
//             stop_time.agency = self.agency.clone();
//             Some(stop_time)
//         } else {
//             None
//         }
//     }
// }

// impl<'a, R: Read + Seek> StopTimeReader<'a, R> {
//     pub fn from_zip(zip: &'a mut zip::ZipArchive<R>, agency: &str) -> Self {
//         let file = zip.by_name("stop_times.txt")
//             .expect("zip did not have stop_times.txt");
//         let mut reader = csv::ReaderBuilder::new()
//             .trim(csv::Trim::Fields)
//             .flexible(true)
//             .from_reader(file);
//         StopTimeReader {
//             agency: String::from(agency),
//             reader,
//             iter: reader.deserialize(),
//         }
//     }
// }

// impl FromZip for BTreeMap<GtfsTime, StopTime> {
//     fn from_zip<R: Read + Seek>(zip: &mut zip::ZipArchive<R>, agency: &str) -> Self {
//         let reader = StopTimeReader::from_zip(zip, agency);
//         let mut stop_times = BTreeMap::new();
//         for stop_time in reader {
//             stop_times.insert(stop_time.departure_time, stop_time);
//         }
//         stop_times
//     }
// }

impl<R: Read + Seek> FromZip<R> for Vec<StopTime> {
    fn from_zip(zip: &mut zip::ZipArchive<R>, agency: &str) -> Self {
        let file = zip.by_name("stop_times.txt")
            .expect("zip did not have stop_times.txt");
        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::Fields)
            .flexible(true)
            .from_reader(file);
        let mut stop_times = Vec::new();
        for result in reader.deserialize() {
            let mut stop_time: StopTime = result
                .expect("could not deserialize stop time");
            stop_time.agency = String::from(agency);
            stop_times.push(stop_time);
        }
        stop_times
    }
}