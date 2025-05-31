use anyhow::Result;

const GRT_GTFS: &str = "https://www.regionofwaterloo.ca/opendatadownloads/GRT_GTFS.zip";

pub async fn download_zip<'a>(
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
    let client = reqwest::Client::new();
    let zip = download_zip(&client, GRT_GTFS).await?;
    Ok(())
}