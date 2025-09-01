mod error;
mod formatter;
mod gtfs_rt;

use std::{collections::HashMap};
use actix_web::{get, web, App, HttpServer};
use anyhow::Result;
use clorinde::{deadpool_postgres::{Config, CreatePoolError, Pool, Runtime}, queries::combined::get_next_deps_near_point, tokio_postgres::NoTls};
use futures::{stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use crate::formatter::FormattedData;

struct AppState {
    cfg: HashMap<String, config::Agency>,
    pool: Pool,
    req_client: reqwest::Client,
}

#[get("/departures-after-dt/{dt}/{lat}/{lon}/{limit}")]
async fn get_departures_after_dt(path: web::Path<(i64, f64, f64, u64)>, data: web::Data<AppState>) -> error::Result<web::Json<Vec<FormattedData>>> {
    let db_client = data.pool.get().await?;
    let (dt, lat, lon, limit) = path.into_inner();
    let chrono_dt = chrono::DateTime::from_timestamp(dt, 0)
        .ok_or_else(|| error::Error::InvalidURLError("Timestamp could not be parsed into a datetime".to_string()))?
        .naive_utc();
    let now = gtfs::GtfsTime::from_chrono_time(chrono_dt);

    let result = get_next_deps_near_point()
        .bind(&db_client, &chrono_dt.date(), &lat, &lon, &(limit as i64), &now.into())
        .all()
        .await?;

    // collect agencies to get updates from
    let agencies = result.iter().map(|r| &r.agency).unique();
    // map of {agency name: realtime updates for that agency}
    let updates = agencies.map(async |a| (a, gtfs_rt::get_agency_updates(&data.cfg, a, &data.req_client).await))
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

    gtfs_rt::apply_updates_to_formatted_data_list(&now, &updates, &mut formatted_data);

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
                .service(get_departures_after_dt)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?)
}