use std::{collections::HashMap, fs::File, io::Read};
use anyhow::Result;

const CONFIG_FILE: &str = "config.toml";

#[derive(serde::Deserialize)]
pub struct Agency {
    pub gtfs_url: String,
}

pub fn get_config(s: &str) -> Result<HashMap<String, Agency>> {
    Ok(toml::from_str(s)?)
}

pub fn read_config_from_file() -> Result<HashMap<String, Agency>> {
    let mut file = File::open(CONFIG_FILE)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    get_config(&buffer)
}

mod test {
    #[test]
    fn get_config() {
        let result = super::get_config(r#"
            [grt]
            gtfs_url = "abc"

            [ttc]
            gtfs_url = "def""#).unwrap();
        assert_eq!(result.get("grt").unwrap().gtfs_url, "abc");
        assert_eq!(result.get("ttc").unwrap().gtfs_url, "def");
    }
}