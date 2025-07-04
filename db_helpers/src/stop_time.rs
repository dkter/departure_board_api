use clorinde::queries::stop_times::{InsertStopTimeParams, StopTimes};

pub fn stop_time_to_db_record(stop_time: gtfs::StopTime) -> InsertStopTimeParams<String, String, String, String, String, String, String, String> {
    InsertStopTimeParams {
        agency: stop_time.agency,
        departure_time: stop_time.departure_time.map(|t| t.into()),
        end_pickup_drop_off_window: stop_time.end_pickup_drop_off_window.map(|t| t.into()),
        arrival_time: stop_time.arrival_time.map(|t| t.into()),
        start_pickup_drop_off_window: stop_time.start_pickup_drop_off_window.map(|t| t.into()),
        trip_id: stop_time.trip_id,
        stop_id: stop_time.stop_id,
        location_group_id: stop_time.location_group_id,
        location_id: stop_time.location_id,
        stop_sequence: stop_time.stop_sequence,
        stop_headsign: stop_time.stop_headsign,
        pickup_type: stop_time.pickup_type,
        drop_off_type: stop_time.drop_off_type,
        continuous_pickup: stop_time.continuous_pickup,
        continuous_drop_off: stop_time.continuous_drop_off,
        shape_dist_traveled: stop_time.shape_dist_traveled,
        timepoint: stop_time.timepoint,
        pickup_booking_rule_id: stop_time.pickup_booking_rule_id,
        drop_off_booking_rule_id: stop_time.drop_off_booking_rule_id,
    }
}

pub fn db_record_to_stop_time(db_record: StopTimes) -> gtfs::StopTime {
    gtfs::StopTime {
        agency: db_record.agency,
        trip_id: db_record.trip_id,
        arrival_time: db_record.arrival_time.map(|t| t.into()),
        departure_time: db_record.departure_time.map(|t| t.into()),
        stop_id: db_record.stop_id,
        location_group_id: db_record.location_group_id,
        location_id: db_record.location_id,
        stop_sequence: db_record.stop_sequence,
        stop_headsign: db_record.stop_headsign,
        start_pickup_drop_off_window: db_record.start_pickup_drop_off_window.map(|t| t.into()),
        end_pickup_drop_off_window: db_record.end_pickup_drop_off_window.map(|t| t.into()),
        pickup_type: db_record.pickup_type,
        drop_off_type: db_record.drop_off_type,
        continuous_pickup: db_record.continuous_pickup,
        continuous_drop_off: db_record.continuous_drop_off,
        shape_dist_traveled: db_record.shape_dist_traveled,
        timepoint: db_record.timepoint,
        pickup_booking_rule_id: db_record.pickup_booking_rule_id,
        drop_off_booking_rule_id: db_record.drop_off_booking_rule_id,
    }
}