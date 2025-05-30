--: StopTimes(arrival_time?, departure_time?, stop_id?, location_group_id?, location_id?, stop_headsign?, start_pickup_drop_off_window?, end_pickup_drop_off_window?, pickup_type?, drop_off_type?, continuous_pickup?, continuous_drop_off?, shape_dist_traveled?, timepoint?, pickup_booking_rule_id?, drop_off_booking_rule_id?)

--! insert_stop_time(arrival_time?, departure_time?, stop_id?, location_group_id?, location_id?, stop_headsign?, start_pickup_drop_off_window?, end_pickup_drop_off_window?, pickup_type?, drop_off_type?, continuous_pickup?, continuous_drop_off?, shape_dist_traveled?, timepoint?, pickup_booking_rule_id?, drop_off_booking_rule_id?)
INSERT INTO StopTimes (
    Agency,
    SortableTime,
    trip_id,
    arrival_time,
    departure_time,
    stop_id,
    location_group_id,
    location_id,
    stop_sequence,
    stop_headsign,
    start_pickup_drop_off_window,
    end_pickup_drop_off_window,
    pickup_type,
    drop_off_type,
    continuous_pickup,
    continuous_drop_off,
    shape_dist_traveled,
    timepoint,
    pickup_booking_rule_id,
    drop_off_booking_rule_id
) VALUES (
    :agency,
    COALESCE(:departure_time, :end_pickup_drop_off_window, :arrival_time, :start_pickup_drop_off_window, -1),
    :trip_id,
    :arrival_time,
    :departure_time,
    :stop_id,
    :location_group_id,
    :location_id,
    :stop_sequence,
    :stop_headsign,
    :start_pickup_drop_off_window,
    :end_pickup_drop_off_window,
    :pickup_type,
    :drop_off_type,
    :continuous_pickup,
    :continuous_drop_off,
    :shape_dist_traveled,
    :timepoint,
    :pickup_booking_rule_id,
    :drop_off_booking_rule_id
);

--! get_next_departures_after_time : StopTimes
SELECT * FROM StopTimes
WHERE SortableTime > :time
    AND stop_id = :stop_id 
ORDER BY SortableTime ASC
LIMIT :limit;