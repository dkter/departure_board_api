/// GTFS Trip struct that supports deserializing from trips.txt via serde and csv.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Trip {
    #[serde(skip_deserializing)]
    pub agency: String,
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
    pub direction_id: Option<i32>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    pub wheelchair_accessible: Option<i32>,
    pub bikes_allowed: Option<i32>,
}