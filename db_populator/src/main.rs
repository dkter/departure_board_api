use anyhow::Result;
use clorinde::queries::agencies::{get_agency_checksum, insert_agency, delete_agency};
use clorinde::tokio_postgres;
use clorinde::deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use db_helpers::copy::write_to_table;

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

            let mut stop_times_zip = zip.clone();
            let stop_times = gtfs::read_gtfs_objects_from_zip(&mut stop_times_zip, &agency_name)?;
            // Ignore/warn on all errors
            let stop_times = stop_times.map(|res| res.expect("Attempting to insert invalid stop time into database"));
            let stop_times_fut = write_to_table::<gtfs::StopTime>(stop_times, &transaction);

            let mut trips_zip = zip.clone();
            let trips = gtfs::read_gtfs_objects_from_zip(&mut trips_zip, &agency_name)?;
            let trips = trips.map(|res| res.expect("Attempting to insert invalid trip into database"));
            let trips_fut = write_to_table::<gtfs::Trip>(trips, &transaction);

            let mut stops_zip = zip.clone();
            let stops = gtfs::read_gtfs_objects_from_zip(&mut stops_zip, &agency_name)?;
            let stops = stops.map(|res| res.expect("Attempting to insert invalid stop into database"));
            let stops_fut = write_to_table::<gtfs::Stop>(stops, &transaction);

            let mut routes_zip = zip.clone();
            let routes = gtfs::read_gtfs_objects_from_zip(&mut routes_zip, &agency_name)?;
            let routes = routes.map(|res| res.expect("Attempting to insert invalid route into database"));
            let routes_fut = write_to_table::<gtfs::Route>(routes, &transaction);

            let mut calendar_zip = zip.clone();
            let calendar = gtfs::read_gtfs_objects_from_zip(&mut calendar_zip, &agency_name);
            let calendar_fut = async {
                match calendar {
                    Ok(calendar) => {
                        let calendar = calendar.map(|res| res.expect("Attempting to insert invalid calendar into database"));
                        write_to_table::<gtfs::Calendar>(calendar, &transaction).await
                    },
                    // Ignore file not found errors, since calendar is optional and calendar_dates may be used instead
                    Err(e) => {
                        if let Some(zip::result::ZipError::FileNotFound) = e.downcast_ref::<zip::result::ZipError>() {
                            Ok(0)
                        } else {
                            Err(e)
                        }
                    },
                }
            };

            let mut calendar_dates_zip = zip.clone();
            let calendar_dates = gtfs::read_gtfs_objects_from_zip(&mut calendar_dates_zip, &agency_name);
            let calendar_dates_fut = async {
                match calendar_dates {
                    Ok(calendar_dates) => {
                        let calendar_dates = calendar_dates.map(|res| res.expect("Attempting to insert invalid calendar_dates into database"));
                        write_to_table::<gtfs::CalendarDate>(calendar_dates, &transaction).await
                    },
                    // Ignore file not found errors, since calendar_dates is optional and calendar may be used instead
                    Err(e) => {
                        if let Some(zip::result::ZipError::FileNotFound) = e.downcast_ref::<zip::result::ZipError>() {
                            Ok(0)
                        } else {
                            Err(e)
                        }
                    },
                }
            };

            let results = futures::try_join!(stop_times_fut, trips_fut, stops_fut, routes_fut, calendar_fut, calendar_dates_fut)?;
            let rows_affected = results.0 + results.1 + results.2 + results.3 + results.4 + results.5;

            transaction.commit().await?;

            println!("Inserted agency {} with {} rows", agency_name, rows_affected);
        }
    }

    // Recreate deleted indexes
    clorinde::queries::stop_times::create_index().bind(&db_client).await?;
    clorinde::queries::stops::create_index().bind(&db_client).await?;

    Ok(())
}