use clorinde::queries::stops::InsertStopParams;

pub fn stop_to_db_record(stop: gtfs::Stop) -> InsertStopParams<String, String, String, String, String, String, String, String, String, String, String, String, String> {
    InsertStopParams {
        agency: stop.agency,
        stop_id: stop.stop_id,
        stop_code: stop.stop_code,
        stop_name: stop.stop_name,
        tts_stop_name: stop.tts_stop_name,
        stop_desc: stop.stop_desc,
        stop_lat: stop.stop_lat,
        stop_lon: stop.stop_lon,
        zone_id: stop.zone_id,
        stop_url: stop.stop_url,
        location_type: stop.location_type,
        parent_station: stop.parent_station,
        stop_timezone: stop.stop_timezone,
        wheelchair_boarding: stop.wheelchair_boarding.map(|n| n as i32),
        level_id: stop.level_id,
        platform_code: stop.platform_code,
    }
}