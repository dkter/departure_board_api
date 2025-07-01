--: DepartureResult(stop_code?, route_short_name?, route_long_name?, route_color?, route_text_color?, route_type?, trip_headsign?, direction_id?)

--! get_next_deps_near_point : DepartureResult
WITH next_deps_of_nearest_stops AS (
    WITH n_nearest_stops AS (
        SELECT *
        FROM Stops
        ORDER BY stop_lat_lon <-> point (:lat, :lon)
        LIMIT :num_stops
    )
    SELECT
        *,
        ROW_NUMBER() OVER (PARTITION BY (agency, route_id, direction_id) ORDER BY SortableTime ASC) nth_of_route
    FROM StopTimes
    JOIN n_nearest_stops USING (agency, stop_id)
    JOIN trips USING (agency, trip_id)
    WHERE SortableTime >= :time AND SortableTime < :time + 7200
)
SELECT
    agency, sortabletime, timezone, stop_id, stop_code, stop_name,
    stop_lat_lon[0] as stop_lat,
    stop_lat_lon[1] as stop_lon,
    route_id, route_short_name, route_long_name, route_color, route_text_color, route_type,
    trip_id, trip_headsign, direction_id
FROM next_deps_of_nearest_stops
JOIN routes USING (agency, route_id)
JOIN agencies USING (agency)
WHERE nth_of_route = 1;