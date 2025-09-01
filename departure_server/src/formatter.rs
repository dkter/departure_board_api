use std::borrow::Cow;

use clorinde::queries::combined::DepartureResult;
use titlecase::Titlecase;

#[derive(Debug, serde::Serialize)]
pub struct FormattedData {
    pub agency: String,
    pub stop_id: String,
    pub route_id: String,
    pub trip_id: String,
    pub time: u32,
    pub timezone: chrono_tz::Tz,
    pub stop_name: String,
    pub dest_name: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub vehicle_type: u32,
    pub fg_colour: String,
    pub bg_colour: String,
    pub shape: u32,
}

pub trait Formatter {
    fn format(&self, db_record: &DepartureResult) -> FormattedData {
        FormattedData {
            agency: db_record.agency.clone(),
            stop_id: db_record.stop_id.clone(),
            route_id: db_record.route_id.clone(),
            trip_id: db_record.trip_id.clone(),
            time: db_record.sortabletime as u32,
            timezone: db_record.timezone.parse().expect("Unexpected value for timezone"),
            stop_name: self.get_stop_name(db_record),
            dest_name: self.get_dest_name(db_record),
            route_short_name: self.get_route_short_name(db_record),
            route_long_name: self.get_route_long_name(db_record),
            vehicle_type: self.get_vehicle_type(db_record),
            fg_colour: self.get_fg_colour(db_record),
            bg_colour: self.get_bg_colour(db_record),
            shape: self.get_shape(db_record),
        }
    }

    fn get_stop_name(&self, db_record: &DepartureResult) -> String {
        db_record.stop_name.clone()
    }

    fn get_dest_name(&self, db_record: &DepartureResult) -> String {
        db_record.trip_headsign
            .clone()
            .unwrap_or("".to_string())
    }

    fn get_route_short_name(&self, db_record: &DepartureResult) -> String {
        db_record.route_short_name
            .clone()
            .unwrap_or(db_record.route_id.clone())
    }

    fn get_route_long_name(&self, db_record: &DepartureResult) -> String {
        db_record.route_long_name
            .clone()
            .expect(&format!("Agency {} does not publish route_long_name - find alternative", db_record.agency))
    }

    fn get_vehicle_type(&self, db_record: &DepartureResult) -> u32 {
        db_record.route_type
            .expect(&format!("Agency {} does not publish route_type - find alternative", db_record.agency))
            as u32
    }

    fn get_fg_colour(&self, db_record: &DepartureResult) -> String {
        db_record.route_text_color
            .clone()
            .unwrap_or_else(|| "ffffff".to_string())
    }

    fn get_bg_colour(&self, db_record: &DepartureResult) -> String {
        db_record.route_color
            .clone()
            .unwrap_or_else(|| "000000".to_string())
    }

    fn get_shape(&self, _db_record: &DepartureResult) -> u32 {
        0
    }
}

struct Grt;
impl Formatter for Grt {
    fn get_bg_colour(&self, db_record: &DepartureResult) -> String {
        match &db_record.route_color {
            Some(colour) => colour.clone(),
            None => match db_record.route_short_name.as_deref().unwrap_or("") {
                // iXpress is green
                "201" | "202" | "203" | "204" | "205" | "206" | "207" | "208" => "55AA00".to_string(),
                // ION is blue
                "301" | "302" | "303" | "304" => "00AAFF".to_string(),
                // ION replacement is red
                "301R" => "FF0000".to_string(),
                // otherwise... grey?
                _ => "555555".to_string(),
            }
        }
    }

    fn get_vehicle_type(&self, db_record: &DepartureResult) -> u32 {
        match db_record.route_short_name.as_deref().unwrap_or("") {
            // GRT's GTFS marks this as heavy rail (2), I'm switching it to 0 (tram)
            "301" => 0,
            _ => db_record.route_type
                .expect(&format!("Agency {} does not publish route_type - find alternative", db_record.agency))
                as u32,
        }
    }
}

struct Ttc;
impl Formatter for Ttc {
    fn get_dest_name(&self, db_record: &DepartureResult) -> String {
        // in the format "EAST - 506 CARLTON towards MAIN STREET STATION"
        let headsign = db_record.trip_headsign.clone().unwrap_or_else(|| return "".to_string());
        let mut split = headsign.split(" towards ");
        let route = split.next().unwrap();
        let towards = split.next().expect("headsign did not have 'towards'");
        let direction = route.split(" - ").next().unwrap();
        format!("{} to {}", direction.titlecase(), towards.to_lowercase().titlecase())
    }

    fn get_stop_name(&self, db_record: &DepartureResult) -> String {
        // shorten from "Gerrard St East at Carlaw Ave" to "Gerrard / Carlaw"
        let street_suffix = regex::Regex::new(" (St|Av|Ave|Dr|Rd|Blvd)( East| West)? ").unwrap();
        let stop_name = db_record.stop_name.replace(" at ", " / ");
        match street_suffix.replace_all(&stop_name, " ") {
            Cow::Borrowed(_) => stop_name,
            Cow::Owned(s) => s,
        }
    }

    fn get_route_long_name(&self, db_record: &DepartureResult) -> String {
        db_record.route_long_name.as_ref().unwrap().titlecase()
    }
}

pub fn get_formatter_from_agency(agency: &str) -> Box<dyn Formatter> {
    match agency {
        "grt" => Box::new(Grt),
        "ttc" => Box::new(Ttc),
        _ => panic!("Formatter does not exist for agency {}", agency),
    }
}