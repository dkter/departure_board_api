--: Trips(trip_headsign?, trip_short_name?, direction_id?, block_id?, shape_id?, wheelchair_accessible?, bikes_allowed?)

--! insert_trip(trip_headsign?, trip_short_name?, direction_id?, block_id?, shape_id?, wheelchair_accessible?, bikes_allowed?)
INSERT INTO Trips (
    Agency,
    route_id,
    service_id,
    trip_id,
    trip_headsign,
    trip_short_name,
    direction_id,
    block_id,
    shape_id,
    wheelchair_accessible,
    bikes_allowed
) VALUES (
    :agency,
    :route_id,
    :service_id,
    :trip_id,
    :trip_headsign,
    :trip_short_name,
    :direction_id,
    :block_id,
    :shape_id,
    :wheelchair_accessible,
    :bikes_allowed
);