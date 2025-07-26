mod error;
mod formatter;

use std::{collections::HashMap};
use actix_web::{get, web, App, HttpServer};
use anyhow::Result;
use clorinde::{deadpool_postgres::{Config, CreatePoolError, Pool, Runtime}, queries::combined::get_next_deps_near_point, tokio_postgres::NoTls};
use futures::{stream::FuturesUnordered, StreamExt};
use gtfs::GtfsTime;
use crate::formatter::{Formatter, FormattedData};
use itertools::Itertools;
use prost::Message;

struct AppState {
    cfg: HashMap<String, config::Agency>,
    pool: Pool,
    req_client: reqwest::Client,
}

async fn get_agency_updates(
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

fn apply_updates_to_formatted_data_list(
    now: &gtfs::GtfsTime,
    updates: &HashMap<&String, gtfs_rt::FeedMessage>,
    formatted_data: &mut Vec<FormattedData>,
) {
    for fd in formatted_data {
        if let Some(update) = updates.get(&fd.agency) {
            for entity in &update.entity {
                // handle trip updates
                if let Some(trip_update) = &entity.trip_update {
                    if trip_update.trip.trip_id.as_ref().is_some_and(|trip_id| trip_id == &fd.trip_id) {
                        // If this trip update mentions our stop, update the departure time of that stop
                        for stop_time_update in &trip_update.stop_time_update {
                            if stop_time_update.stop_id.as_ref().is_some_and(|stop_id| stop_id == &fd.stop_id) {
                                if let Some(updated_time) = get_updated_departure_time(stop_time_update, &fd.timezone) {
                                    println!("dbg: adjusting time {:?} -> {:?} for trip {}",
                                            GtfsTime::from(fd.time), updated_time, fd.trip_id);
                                    fd.time = updated_time.into();
                                }
                            }
                        }
                    } else if trip_update.trip.route_id.as_ref().is_some_and(|route_id| route_id == &fd.route_id) {
                        // If this trip update has a departure after the current time but before the scheduled departure,
                        // replace the scheduled departure with that one
                        for stop_time_update in &trip_update.stop_time_update {
                            if stop_time_update.stop_id.as_ref().is_some_and(|stop_id| stop_id == &fd.stop_id) {
                                if let Some(updated_time) = get_updated_departure_time(stop_time_update, &fd.timezone) {
                                    if now < &updated_time && updated_time < fd.time.into() {
                                        println!("dbg: adjusting time {:?} -> {:?} for trip {}",
                                                GtfsTime::from(fd.time), updated_time, fd.trip_id);
                                        fd.time = updated_time.into();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[get("/departures/{lat}/{lon}/{limit}")]
async fn get_departures(path: web::Path<(f64, f64, u64)>, data: web::Data<AppState>) -> error::Result<web::Json<Vec<FormattedData>>> {
    let db_client = data.pool.get().await?;
    let (lat, lon, limit) = path.into_inner();
    let now = gtfs::GtfsTime::local_now();

    let result = get_next_deps_near_point().bind(&db_client, &lat, &lon, &(limit as i64), &now.into()).all().await?;

    // collect agencies to get updates from
    let agencies = result.iter().map(|r| &r.agency).unique();
    // map of {agency name: realtime updates for that agency}
    let updates = agencies.map(async |a| (a, get_agency_updates(&data.cfg, a, &data.req_client).await))
        .collect::<FuturesUnordered<_>>()
        .filter_map(async |(agency_name, agency_updates)| {
            // if there's an error, just log and don't include updates for that agency
            match agency_updates {
                Ok(agency_updates) => Some((agency_name, agency_updates)),
                Err(e) => {
                    println!("warning: could not retrieve updates for agency {} due to error: {}", agency_name, e);
                    None
                }
            }
        })
        .collect::<HashMap<_, _>>()
        .await;

    let mut formatted_data = result.iter().map(|r| {
        let f = formatter::get_formatter_from_agency(&r.agency);
        f.format(r)
    }).collect::<Vec<_>>();

    apply_updates_to_formatted_data_list(&now, &updates, &mut formatted_data);

    Ok(web::Json(formatted_data))
}

async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("departure_board"));
    cfg.password = Some(String::from("db"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("departure_board"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::read_config_from_file()?;
    let pool = create_pool().await?;

    Ok(
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState {
                    cfg: cfg.clone(),
                    pool: pool.clone(),
                    req_client: reqwest::Client::new(),
                }))
                .service(get_departures)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?)
}