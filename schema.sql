CREATE TABLE Agencies (
    Agency varchar(32) NOT NULL PRIMARY KEY,
    checksum bigint NOT NULL,
    timezone text NOT NULL
);

CREATE TABLE StopTimes (
    Agency varchar(32) NOT NULL REFERENCES Agencies ON DELETE CASCADE,
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
    Agency varchar(32) NOT NULL REFERENCES Agencies ON DELETE CASCADE,
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

CREATE TABLE Stops (
    Agency varchar(32) NOT NULL REFERENCES Agencies ON DELETE CASCADE,
    stop_id text NOT NULL PRIMARY KEY,
    stop_code text,
    stop_name text NOT NULL,
    tts_stop_name text,
    stop_desc text,
    stop_lat_lon point NOT NULL,
    zone_id text,
    stop_url text,
    location_type text,
    parent_station text,
    stop_timezone text,
    wheelchair_boarding int,
    level_id text,
    platform_code text
);

CREATE INDEX StopPointsIndex ON Stops USING SPGIST (stop_lat_lon);

CREATE TABLE Routes (
    Agency varchar(32) NOT NULL REFERENCES Agencies ON DELETE CASCADE,
    route_id text NOT NULL PRIMARY KEY,
    agency_id text,
    route_short_name text,
    route_long_name text,
    route_desc text,
    route_type int,
    route_url text,
    route_color char(6),
    route_text_color char(6),
    route_sort_order int,
    continuous_pickup int,
    continuous_drop_off int,
    network_id text
);