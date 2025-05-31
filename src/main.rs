mod gtfs;
mod stop_tree;
mod stop_times;

use std::{error::Error, num::NonZero};
use prost::Message;
use clorinde::deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use clorinde::tokio_postgres::NoTls;
use stop_times::{download_feed_and_populate_db, get_next_n_deps};

async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("departure_board"));
    cfg.password = Some(String::from("db"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("departure_board"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}

const GRT_GTFS: &str = "https://www.regionofwaterloo.ca/opendatadownloads/GRT_GTFS.zip";
const GRT_GTFS_RT_POS: &str = "http://webapps.regionofwaterloo.ca/api/grt-routes/api/vehiclepositions";
const GRT_GTFS_RT_UPD: &str = "http://webapps.regionofwaterloo.ca/api/grt-routes/api/tripupdates";

async fn fetch_gtfs_rt(client: &reqwest::Client) -> Result<gtfs_rt::FeedMessage, Box<dyn Error>> {
    let resp = client.get(GRT_GTFS_RT_UPD)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36")
        .send()
        .await?;
    let content = resp.bytes().await?;
    Ok(gtfs_rt::FeedMessage::decode(content)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    // let resp = client.get(GRT_GTFS_RT_UPD)
    //     .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36")
    //     .send()
    //     .await?;
    // let content = resp.bytes().await?;
    // let message = gtfs_rt::FeedMessage::decode(content)?;
    // println!("{:#?}", message);

    // stop_tree::download_and_pack_feed(&client, "grt", GRT_GTFS).await?;

    // let feed_message = await fetch_gtfs_rt(&client)?;

    //let stop_time_vec = stop_times::download_feed(&client, "grt", GRT_GTFS);

    let pool = create_pool().await?;
    let mut db_client = pool.get().await?;
    //download_feed_and_populate_db(&client, &mut db_client, "grt", GRT_GTFS).await?;

    let ast = stop_tree::ArchivedStopTree::unpack_from_files()?;
    let stops = ast.find_nearest(43.457787, -80.513526, NonZero::new(5).unwrap());
    //println!("{:?}", stops);

    let now = gtfs::GtfsTime::local_now();

    for stop in stops {
        //let cursor = stop_time_map.lower_bound
        let next_5_deps = get_next_n_deps(&mut db_client, now, &stop.stop_id, 5).await?;
        println!("{:?}", next_5_deps);
    }

    Ok(())
}
