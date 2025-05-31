use crate::gtfs::stop::Stop;
use crate::gtfs::stop_time::StopTime;
use std::io::{Read, Seek};

pub trait GtfsReadable: serde::de::DeserializeOwned {
    fn get_file_name() -> &'static str;
    fn set_agency(&mut self, agency: String);
}

impl GtfsReadable for Stop {
    fn get_file_name() -> &'static str {
        "stops.txt"
    }

    fn set_agency(&mut self, agency: String) {
        self.agency = agency;
    }
}

impl GtfsReadable for StopTime {
    fn get_file_name() -> &'static str {
        "stop_times.txt"
    }

    fn set_agency(&mut self, agency: String) {
        self.agency = agency;
    }
}

pub fn read_gtfs_objects_from_zip<'a, T: GtfsReadable + 'a, R: Read + Seek>(
    zip: &'a mut zip::ZipArchive<R>, agency: &'a str
) -> impl Iterator<Item = T> + 'a {
    let file_name = T::get_file_name();
    let file = zip.by_name(file_name)
        .expect(&format!("zip did not have file {}", file_name));
    let reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::Fields)
        .flexible(true)
        .from_reader(file);
    reader.into_deserialize().map(move |result| {
        let mut item: T = result
            .expect(&format!("could not deserialize T from {}", file_name));
        item.set_agency(agency.to_string());
        item
    })
}