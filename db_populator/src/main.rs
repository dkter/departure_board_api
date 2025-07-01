use anyhow::Result;
use clorinde::client::Params;
use clorinde::queries::{
    agencies::{get_agency_checksum, insert_agency, delete_agency},
    stop_times::insert_stop_time,
    trips::insert_trip,
    stops::insert_stop,
    routes::insert_route,
};
use clorinde::tokio_postgres;
use clorinde::deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use db_helpers::{
    stop_time::stop_time_to_db_record,
    trip::trip_to_db_record,
    stop::stop_to_db_record,
    route::route_to_db_record,
};
use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};

async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("departure_board"));
    cfg.password = Some(String::from("db"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("departure_board"));
    cfg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
}

async fn download_zip<'a>(
    client: &reqwest::Client,
    url: &str,
) -> Result<zip::ZipArchive<std::io::Cursor<bytes::Bytes>>> {
    let resp = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36")
        .send()
        .await?;
    let content = resp.bytes().await?;

    let reader = std::io::Cursor::new(content);
    let zip_archive = zip::ZipArchive::new(reader)?;
    Ok(zip_archive)
}

fn get_zip_checksum(zip_archive: &mut zip::ZipArchive<std::io::Cursor<bytes::Bytes>>) -> u64 {
    let mut digest = crc_fast::Digest::new(crc_fast::CrcAlgorithm::Crc32IsoHdlc);
    std::io::copy(&mut zip_archive.clone().into_inner(), &mut digest).unwrap();
    digest.finalize()
}

async fn get_db_checksum(db_client: &mut tokio_postgres::Client, agency_name: &str) -> Result<Option<u64>> {
    let result = get_agency_checksum().bind(db_client, &agency_name).all().await?;
    Ok(match result.len() {
        1 => Some(result[0] as u64),
        0 => None,
        _ => panic!("Unexpected number of results returned by get_agency_checksum"),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::read_config_from_file()?;
    let client = reqwest::Client::new();
    let pool = create_pool().await?;
    let mut db_client = pool.get().await?;

    // Temporarily delete indexes for performance
    clorinde::queries::stop_times::delete_index().bind(&db_client).await?;
    clorinde::queries::stops::delete_index().bind(&db_client).await?;

    for (agency_name, agency_cfg) in cfg {
        let (zip, checksum) = tokio::join!(
            download_zip(&client, &agency_cfg.gtfs_url),
            get_db_checksum(&mut db_client, &agency_name),
        );
        let mut zip = zip?;
        let zip_checksum = get_zip_checksum(&mut zip);

        let should_insert = match checksum? {
            Some(c) if zip_checksum == c => false,  // Found and checksum correct
            None => true,  // Not found
            Some(_) => {
                // There is an agency with the given name already in the db,
                // but with an out-of-date checksum
                delete_agency().bind(&mut db_client, &agency_name).await?;
                true
            },
        };
        
        if should_insert {
            let mut transaction = db_client.transaction().await?;

            let timezone = {
                let mut gtfs_agencies = gtfs::read_gtfs_objects_from_zip(&mut zip, &agency_name)?;
                // According to the GTFS reference, if multiple agencies are specifed in agency.txt,
                // they must all have the same agency_timezone.
                // The distinctions between items in agency.txt don't matter to us beyond this, so we
                // only need to read the first one
                let first_gtfs_agency: gtfs::Agency = gtfs_agencies.next().unwrap()?;
                first_gtfs_agency.agency_timezone
            };

            insert_agency().bind(
                &mut transaction, &agency_name, &(zip_checksum as i64), &timezone
            ).await?;

            let stop_times = gtfs::read_gtfs_objects_from_zip(&mut zip, &agency_name)?;
            let stop_times_task: FuturesUnordered<_> = stop_times.map(
                |stop_time| {
                    let params = stop_time_to_db_record(
                        stop_time.expect("Attempting to insert invalid stop time into database"));
                    {
                        let transaction = &transaction;
                        async move { insert_stop_time().params(transaction, &params).await }
                    }
                }
            ).collect();

            let trips = gtfs::read_gtfs_objects_from_zip(&mut zip, &agency_name)?;
            let trips_task: FuturesUnordered<_> = trips.map(
                |trip| {
                    let params = trip_to_db_record(
                        trip.expect("Attempting to insert invalid trip into database"));
                    {
                        let transaction = &transaction;
                        async move { insert_trip().params(transaction, &params).await }
                    }
                }
            ).collect();

            let stops = gtfs::read_gtfs_objects_from_zip(&mut zip, &agency_name)?;
            let stops_task: FuturesUnordered<_> = stops.map(
                |stop| {
                    let params = stop_to_db_record(
                        stop.expect("Attempting to insert invalid stop into database"));
                    {
                        let transaction = &transaction;
                        async move { insert_stop().params(transaction, &params).await }
                    }
                }
            ).collect();

            let routes = gtfs::read_gtfs_objects_from_zip(&mut zip, &agency_name)?;
            let routes_task: FuturesUnordered<_> = routes.map(
                |route| {
                    let params = route_to_db_record(
                        route.expect("Attempting to insert invalid route into database"));
                    {
                        let transaction = &transaction;
                        async move { insert_route().params(transaction, &params).await }
                    }
                }
            ).collect();

            let rows_affected: u64 = stop_times_task
                .chain(trips_task)
                .chain(stops_task)
                .chain(routes_task)
                .try_fold(0, |acc, x| async move { Ok(acc + x) })
                .await?;

            transaction.commit().await?;

            println!("Inserted agency {} with {} rows", agency_name, rows_affected);
        }
    }

    // Recreate deleted indexes
    clorinde::queries::stop_times::create_index().bind(&db_client).await?;
    clorinde::queries::stops::create_index().bind(&db_client).await?;

    Ok(())
}