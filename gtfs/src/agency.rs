/// GTFS Agency struct that supports deserializing from agency.txt via serde and csv.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Agency {
    #[serde(skip_deserializing)]
    pub agency: String,
    // Ignoring all fields except timezone for now, since the GTFS notion of "agency" doesn't matter much to us
    pub agency_timezone: String,
}