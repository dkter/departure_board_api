--: Stop(stop_code?, tts_stop_name?, stop_desc?, zone_id?, stop_url?, location_type?, parent_station?, stop_timezone?, wheelchair_boarding?, level_id?, platform_code?)

--! insert_stop(stop_code?, tts_stop_name?, stop_desc?, zone_id?, stop_url?, location_type?, parent_station?, stop_timezone?, wheelchair_boarding?, level_id?, platform_code?)
INSERT INTO Stops (
    Agency,
    stop_id,
    stop_code,
    stop_name,
    tts_stop_name,
    stop_desc,
    stop_lat_lon,
    zone_id,
    stop_url,
    location_type,
    parent_station,
    stop_timezone,
    wheelchair_boarding,
    level_id,
    platform_code
) VALUES (
    :agency,
    :stop_id,
    :stop_code,
    :stop_name,
    :tts_stop_name,
    :stop_desc,
    (point (:stop_lat, :stop_lon)),
    :zone_id,
    :stop_url,
    :location_type,
    :parent_station,
    :stop_timezone,
    :wheelchair_boarding,
    :level_id,
    :platform_code
);

--! get_n_nearest_stops : Stop
SELECT
    Agency,
    stop_id,
    stop_code,
    stop_name,
    tts_stop_name,
    stop_desc,
    stop_lat_lon[0] AS stop_lat,
    stop_lat_lon[1] AS stop_lon,
    zone_id,
    stop_url,
    location_type,
    parent_station,
    stop_timezone,
    wheelchair_boarding,
    level_id,
    platform_code
FROM Stops
ORDER BY stop_lat_lon <-> point (:lat, :lon)
LIMIT :limit;