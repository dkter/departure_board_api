mod gtfs;
mod stop_tree;

use std::{error::Error, num::NonZero};
use prost::Message;

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

    let ast = stop_tree::ArchivedStopTree::unpack_from_files()?;
    let stops = ast.find_nearest(43.457787, -80.513526, NonZero::new(5).unwrap());
    println!("{:?}", stops);

    Ok(())
}
