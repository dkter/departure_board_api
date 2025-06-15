use clorinde::queries::combined::DepartureResult;

#[derive(Debug, serde::Serialize)]
pub struct FormattedData {
    pub time: u32,
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
            time: db_record.sortabletime as u32,
            stop_name: Self::get_stop_name(db_record),
            dest_name: Self::get_dest_name(db_record),
            route_short_name: Self::get_route_short_name(db_record),
            route_long_name: Self::get_route_long_name(db_record),
            vehicle_type: Self::get_vehicle_type(db_record),
            fg_colour: Self::get_fg_colour(db_record),
            bg_colour: Self::get_bg_colour(db_record),
            shape: Self::get_shape(db_record),
        }
    }

    fn get_stop_name(db_record: &DepartureResult) -> String {
        db_record.stop_name.clone()
    }

    fn get_dest_name(db_record: &DepartureResult) -> String {
        db_record.trip_headsign
            .clone()
            .expect(&format!("Agency {} does not publish trip_headsign - find alternative", db_record.agency))
    }

    fn get_route_short_name(db_record: &DepartureResult) -> String {
        db_record.route_short_name
            .clone()
            .unwrap_or(db_record.route_id.clone())
    }

    fn get_route_long_name(db_record: &DepartureResult) -> String {
        db_record.route_long_name
            .clone()
            .expect(&format!("Agency {} does not publish route_long_name - find alternative", db_record.agency))
    }

    fn get_vehicle_type(db_record: &DepartureResult) -> u32 {
        db_record.route_type
            .expect(&format!("Agency {} does not publish route_type - find alternative", db_record.agency))
            as u32
    }

    fn get_fg_colour(db_record: &DepartureResult) -> String {
        db_record.route_text_color
            .clone()
            .unwrap_or_else(|| "ffffff".to_string())
    }

    fn get_bg_colour(db_record: &DepartureResult) -> String {
        db_record.route_color
            .clone()
            .unwrap_or_else(|| "000000".to_string())
    }

    fn get_shape(_db_record: &DepartureResult) -> u32 {
        0
    }
}

struct Grt;
impl Formatter for Grt {}

pub fn get_formatter_from_agency(agency: &str) -> impl Formatter {
    match agency {
        "grt" => Grt,
        _ => panic!("Formatter does not exist for agency {}", agency),
    }
}