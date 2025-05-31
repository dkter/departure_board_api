use gtfs::{GtfsTime, StopTime};
use anyhow::Result;
use clorinde::client::Params;
use clorinde::queries::stop_times::{insert_stop_time, get_next_departures_after_time};
use clorinde::tokio_postgres;
use db_helpers::stop_time::{stop_time_to_db_record, db_record_to_stop_time};

pub async fn download_feed<'a>(
    client: &reqwest::Client,
    agency: &str,
    url: &str,
) -> Result<Vec<StopTime>> {
    let resp = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36")
        .send()
        .await?;
    let content = resp.bytes().await?;

    let reader = std::io::Cursor::new(content);
    let mut zip = zip::ZipArchive::new(reader).unwrap();

    let vec = gtfs::read_gtfs_objects_from_zip(&mut zip, agency)?.collect();
    vec
}

pub async fn download_feed_and_populate_db(
    reqwest_client: &reqwest::Client,
    db_client: &mut tokio_postgres::Client,
    agency: &str,
    url: &str,
) -> Result<()> {
    let stop_times = download_feed(reqwest_client, agency, url).await?;

    // let mut join_set = stop_times.drain(..).map(move |stop_time| {
    //     let params = stop_time_to_db_record(stop_time);
    //     insert_stop_time().params(db_client, &params)
    // }).collect::<JoinSet<_>>();

    // while let Some(res) = join_set.join_next().await {
    //     res?;
    // }
    let transaction = db_client.transaction().await?;
    for stop_time in stop_times {
        let params = stop_time_to_db_record(stop_time);
        insert_stop_time().params(&transaction, &params).await?;
    }
    transaction.commit().await?;
    Ok(())
}

pub async fn get_next_n_deps(
    db_client: &mut tokio_postgres::Client,
    time: GtfsTime,
    stop_id: &str,
    limit: i64,
) -> Result<Vec<StopTime>> {
    Ok(get_next_departures_after_time().bind(db_client, &time.into(), &stop_id, &limit).all()
        .await?
        .into_iter()
        .map(|record| db_record_to_stop_time(record))
        .collect())
}
