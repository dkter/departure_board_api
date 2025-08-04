--: DepartureResult(stop_code?, route_short_name?, route_long_name?, route_color?, route_text_color?, route_type?, trip_headsign?, direction_id?)

--! get_next_deps_near_point : DepartureResult
WITH next_deps_of_nearest_stops AS (
    WITH n_nearest_stops AS (
        SELECT *, EXTRACT(dow FROM :date::date) dow
        FROM Stops
        ORDER BY stop_lat_lon <-> point (:lat, :lon)
        LIMIT :limit
    )
    SELECT
        *,
        ROW_NUMBER() OVER (PARTITION BY (agency, route_id, direction_id) ORDER BY stop_lat_lon <-> point (:lat, :lon)) nth_of_route
    FROM StopTimes
    JOIN n_nearest_stops USING (agency, stop_id)
    JOIN trips USING (agency, trip_id)
    LEFT JOIN calendar USING (agency, service_id)
    LEFT JOIN calendardates USING (agency, service_id)
    WHERE (
        SortableTime >= :time AND SortableTime < :time + 7200 AND (
            date = :date OR (
                (dow=0 AND sunday) OR
                (dow=1 AND monday) OR
                (dow=2 AND tuesday) OR
                (dow=3 AND wednesday) OR
                (dow=4 AND thursday) OR
                (dow=5 AND friday) OR
                (dow=6 AND saturday)
            )
        )
    ) OR (
        -- it's tomorrow!
        SortableTime >= 86400 + :time AND SortableTime < 86400 + :time + 7200 AND (
            date = :date::date - interval '1 day' OR (
                (dow=1 AND sunday) OR
                (dow=2 AND monday) OR
                (dow=3 AND tuesday) OR
                (dow=4 AND wednesday) OR
                (dow=5 AND thursday) OR
                (dow=6 AND friday) OR
                (dow=0 AND saturday)
            )
        )
    )
    
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