use clorinde::queries::trips::InsertTripParams;

pub fn trip_to_db_record(trip: gtfs::Trip) -> InsertTripParams<String, String, String, String, String, String, String, String> {
    InsertTripParams {
        agency: trip.agency,
        route_id: trip.route_id,
        service_id: trip.service_id,
        trip_id: trip.trip_id,
        trip_headsign: trip.trip_headsign,
        trip_short_name: trip.trip_short_name,
        direction_id: trip.direction_id,
        block_id: trip.block_id,
        shape_id: trip.shape_id,
        wheelchair_accessible: trip.wheelchair_accessible,
        bikes_allowed: trip.bikes_allowed,
    }
}