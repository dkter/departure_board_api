mod error;
mod formatter;

use actix_web::{get, web, App, HttpServer, Responder};
use clorinde::{deadpool_postgres::{Config, CreatePoolError, Pool, Runtime}, queries::combined::get_next_deps_near_point, tokio_postgres::NoTls};
use crate::formatter::Formatter;

struct AppState {
    pool: Pool,
}

#[get("/departures/{lat}/{lon}/{limit}")]
async fn get_departures(path: web::Path<(f64, f64, u64)>, data: web::Data<AppState>) -> error::Result<impl Responder> {
    let db_client = data.pool.get().await?;
    let (lat, lon, limit) = path.into_inner();
    let now = gtfs::GtfsTime::local_now();
    let result = get_next_deps_near_point().bind(&db_client, &lat, &lon, &(limit as i64), &now.into()).all().await?;

    let formatted_data = result.iter().map(|r| {
        let f = formatter::get_formatter_from_agency(&r.agency);
        f.format(r)
    }).collect::<Vec<_>>();

    Ok(format!("{:?}!", formatted_data))
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
    let pool = create_pool().await?;

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
            }))
            .service(get_departures)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?)
}