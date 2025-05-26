use std::{collections::BTreeMap, error::Error};
use crate::gtfs::{GtfsTime, FromZip, StopTime};

pub async fn download_feed(
    client: &reqwest::Client, agency: &str, url: &str
) -> Result<BTreeMap<GtfsTime, StopTime>, Box<dyn Error>> {
    let resp = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36")
        .send()
        .await?;
    let content = resp.bytes().await?;

    let reader = std::io::Cursor::new(content);
    let mut zip = zip::ZipArchive::new(reader).unwrap();

    Ok(BTreeMap::<GtfsTime, StopTime>::from_zip(&mut zip, agency))
}