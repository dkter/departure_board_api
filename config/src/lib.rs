use std::{collections::HashMap, fs::File, io::Read};
use anyhow::Result;

const CONFIG_FILE: &str = "config.toml";

#[derive(Clone, serde::Deserialize)]
pub struct Agency {
    pub gtfs_url: String,
    pub gtfs_rt_updates_url: String,
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
            gtfs_rt_updates_url = "ABC"

            [ttc]
            gtfs_url = "def"
            gtfs_rt_updates_url = "DEF""#).unwrap();
        assert_eq!(result.get("grt").unwrap().gtfs_url, "abc");
        assert_eq!(result.get("grt").unwrap().gtfs_rt_updates_url, "ABC");
        assert_eq!(result.get("ttc").unwrap().gtfs_url, "def");
        assert_eq!(result.get("ttc").unwrap().gtfs_rt_updates_url, "DEF");
    }
}