use crate::gtfs::{self, GtfsTime, StopTime};
use anyhow::Result;
use clorinde::client::Params;
use clorinde::queries::stop_times::{
    InsertStopTimeParams, insert_stop_time, get_next_departures_after_time};
use clorinde::tokio_postgres;

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

fn stop_time_to_db_record(stop_time: StopTime) -> InsertStopTimeParams<String, String, String, String, String, String, String, String> {
    InsertStopTimeParams {
        agency: stop_time.agency,
        departure_time: stop_time.departure_time.map(|t| t.into()),
        end_pickup_drop_off_window: stop_time.end_pickup_drop_off_window.map(|t| t.into()),
        arrival_time: stop_time.arrival_time.map(|t| t.into()),
        start_pickup_drop_off_window: stop_time.start_pickup_drop_off_window.map(|t| t.into()),
        trip_id: stop_time.trip_id,
        stop_id: stop_time.stop_id,
        location_group_id: stop_time.location_group_id,
        location_id: stop_time.location_id,
        stop_sequence: stop_time.stop_sequence,
        stop_headsign: stop_time.stop_headsign,
        pickup_type: stop_time.pickup_type,
        drop_off_type: stop_time.drop_off_type,
        continuous_pickup: stop_time.continuous_pickup,
        continuous_drop_off: stop_time.continuous_drop_off,
        shape_dist_traveled: stop_time.shape_dist_traveled,
        timepoint: stop_time.timepoint,
        pickup_booking_rule_id: stop_time.pickup_booking_rule_id,
        drop_off_booking_rule_id: stop_time.drop_off_booking_rule_id,
    }
}

fn db_record_to_stop_time(db_record: clorinde::queries::stop_times::StopTimes) -> StopTime {
    StopTime {
        agency: db_record.agency,
        trip_id: db_record.trip_id,
        arrival_time: db_record.arrival_time.map(|t| t.into()),
        departure_time: db_record.departure_time.map(|t| t.into()),
        stop_id: db_record.stop_id,
        location_group_id: db_record.location_group_id,
        location_id: db_record.location_id,
        stop_sequence: db_record.stop_sequence,
        stop_headsign: db_record.stop_headsign,
        start_pickup_drop_off_window: db_record.start_pickup_drop_off_window.map(|t| t.into()),
        end_pickup_drop_off_window: db_record.end_pickup_drop_off_window.map(|t| t.into()),
        pickup_type: db_record.pickup_type,
        drop_off_type: db_record.drop_off_type,
        continuous_pickup: db_record.continuous_pickup,
        continuous_drop_off: db_record.continuous_drop_off,
        shape_dist_traveled: db_record.shape_dist_traveled,
        timepoint: db_record.timepoint,
        pickup_booking_rule_id: db_record.pickup_booking_rule_id,
        drop_off_booking_rule_id: db_record.drop_off_booking_rule_id,
    }
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
