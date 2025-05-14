use std::io::{Read, Seek};

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
