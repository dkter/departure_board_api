use crate::gtfs::gtfs_time::GtfsTime;

/// GTFS Stop Time struct that supports deserializing from stop_times.txt via serde and csv.
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