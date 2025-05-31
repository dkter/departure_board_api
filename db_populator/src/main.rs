use anyhow::Result;
use clorinde::client::Params;
use clorinde::queries::stop_times::insert_stop_time;
use clorinde::tokio_postgres;
use clorinde::deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use db_helpers::stop_time::{stop_time_to_db_record, db_record_to_stop_time};
use futures::stream::FuturesUnordered;
use futures::StreamExt;

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

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::read_config_from_file()?;
    let client = reqwest::Client::new();
    let pool = create_pool().await?;
    let mut db_client = pool.get().await?;

    for (agency_name, agency_cfg) in cfg {
        let mut zip = download_zip(&client, &agency_cfg.gtfs_url).await?;

        let stop_times = gtfs::read_gtfs_objects_from_zip(&mut zip, &agency_name)?;

        let transaction = db_client.transaction().await?;

        let task: FuturesUnordered<_> = stop_times.map(
            |stop_time| {
                let params = stop_time_to_db_record(
                    stop_time.expect("Attempting to insert invalid stop time into database"));
                {
                    let transaction = &transaction;
                    async move { insert_stop_time().params(transaction, &params).await }
                }
            }
        ).collect();
        let _: Vec<_> = task.collect().await;

        transaction.commit().await?;
    }
    Ok(())
}