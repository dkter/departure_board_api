/// GTFS Route struct that supports deserializing from routes.txt via serde and csv.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Route {
    #[serde(skip_deserializing)]
    pub agency: String,
    pub route_id: String,
    pub agency_id: Option<String>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub route_desc: Option<String>,
    pub route_type: Option<i32>,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
    pub route_sort_order: Option<i32>,
    pub continuous_pickup: Option<i32>,
    pub continuous_drop_off: Option<i32>,
    pub network_id: Option<String>,
}