use std::collections::HashMap;
use anyhow::Result;
use prost::Message;
use gtfs::GtfsTime;
use crate::formatter::FormattedData;

pub async fn get_agency_updates(
    cfg: &HashMap<String, config::Agency>,
    agency: &str,
    req_client: &reqwest::Client
) -> Result<gtfs_rt::FeedMessage> {
    let url = &cfg.get(agency).expect("nonexistent agency passed to get_agency_updates").gtfs_rt_updates_url;
    let resp = req_client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36")
        .send()
        .await?;
    let content = resp.bytes().await?;
    let message = gtfs_rt::FeedMessage::decode(content)?;
    Ok(message)
}

fn get_updated_departure_time(
    update: &gtfs_rt::trip_update::StopTimeUpdate,
    timezone: &chrono_tz::Tz
) -> Option<GtfsTime> {
    let stop_time_event = update.departure.as_ref().or(update.arrival.as_ref());
    if let Some(stop_time_event) = stop_time_event {
        if let Some(time) = stop_time_event.time {
            let chrono_time = chrono::DateTime::from_timestamp(time, 0).unwrap()
                .with_timezone(timezone);
            let new_time = GtfsTime::from_chrono_time(chrono_time);
            return Some(new_time);
        }
    }
    None
}

fn time_updated_trip_reaches_stop(
    trip_update: &gtfs_rt::TripUpdate,
    stop_id: &str,
    timezone: &chrono_tz::Tz,
) -> Option<GtfsTime> {
    for stop_time_update in &trip_update.stop_time_update {
        if stop_time_update.stop_id.as_ref().is_some_and(|s| s == stop_id) {
            return get_updated_departure_time(stop_time_update, timezone)
        }
    }
    None
}

/// Within a trip update, finds a stop time update with the given stop_id if it exists,
/// otherwise return null.
fn find_stop_time_update_matching_stop_id<'a>(
    trip_update: &'a gtfs_rt::TripUpdate,
    stop_id: &str,
) -> Option<&'a gtfs_rt::trip_update::StopTimeUpdate> {
    trip_update.stop_time_update
        .iter()
        .find(|stu| stu.stop_id.as_ref().is_some_and(|s| s == stop_id))
}

fn apply_trip_update_to_fd_matching_trip_id(
    now: &GtfsTime,
    trip_update: &gtfs_rt::TripUpdate,
    fd: &mut FormattedData,
) {
    if trip_update.trip.trip_id.as_ref().is_some_and(|trip_id| trip_id == &fd.trip_id) {
        let dep_time = find_stop_time_update_matching_stop_id(&trip_update, &fd.stop_id)
            .and_then(|stu| get_updated_departure_time(stu, &fd.timezone));
        if let Some(updated_time) = dep_time {
            if now < &updated_time && updated_time < fd.time.into() {
                println!("dbg: adjusting time {:?} -> {:?} for trip {}",
                        GtfsTime::from(fd.time), updated_time, fd.trip_id);
                fd.time = updated_time.into();
            }
        }
    }
}

fn apply_trip_update_to_fd_matching_route_id(
    now: &GtfsTime,
    trip_update: &gtfs_rt::TripUpdate,
    fd: &mut FormattedData,
) {
    if trip_update.trip.route_id.as_ref().is_some_and(|route_id| route_id == &fd.route_id) {
        // If this trip update has a departure after the current time but before the scheduled departure,
        // replace the scheduled departure with that one
        if let Some(updated_time) = time_updated_trip_reaches_stop(trip_update, &fd.stop_id, &fd.timezone) {
            if now < &updated_time && updated_time < fd.time.into() {
                println!("dbg: adjusting time {:?} -> {:?} for trip {}",
                        GtfsTime::from(fd.time), updated_time, fd.trip_id);
                fd.time = updated_time.into();
            }
        }
    }
}

pub fn apply_updates_to_formatted_data_list(
    now: &GtfsTime,
    updates: &HashMap<&String, gtfs_rt::FeedMessage>,
    formatted_data: &mut Vec<FormattedData>,
) {
    for fd in formatted_data {
        if let Some(update) = updates.get(&fd.agency) {
            for entity in &update.entity {
                // handle trip updates
                if let Some(trip_update) = &entity.trip_update {
                    apply_trip_update_to_fd_matching_trip_id(now, trip_update, fd);
                }
            }
        }
    }
}