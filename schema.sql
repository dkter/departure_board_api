CREATE TABLE StopTimes (
    Agency varchar(32) NOT NULL,
    SortableTime int NOT NULL,
    trip_id text NOT NULL,
    arrival_time int,
    departure_time int,
    stop_id text,
    location_group_id text,
    location_id text,
    stop_sequence int NOT NULL,
    stop_headsign text,
    start_pickup_drop_off_window int,
    end_pickup_drop_off_window int,
    pickup_type int,
    drop_off_type int,
    continuous_pickup int,
    continuous_drop_off int,
    shape_dist_traveled real,
    timepoint int,
    pickup_booking_rule_id text,
    drop_off_booking_rule_id text
);

CREATE INDEX SortableTimeIndex ON StopTimes USING HASH (SortableTime);

CREATE TABLE Trips (
    Agency varchar(32) NOT NULL,
    route_id text NOT NULL,
    service_id text NOT NULL,
    trip_id text NOT NULL PRIMARY KEY,
    trip_headsign text,
    trip_short_name text,
    direction_id int,
    block_id text,
    shape_id text,
    wheelchair_accessible int,
    bikes_allowed int
);