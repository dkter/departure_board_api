/// Time type used in GTFS structs, with the format `GtfsTime(hour, minute, second)`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GtfsTime(u8, u8, u8);

impl GtfsTime {
    pub fn from_chrono_time(ch: impl chrono::Timelike) -> Self {
        Self(ch.hour() as u8, ch.minute() as u8, ch.second() as u8)
    }

    pub fn local_now() -> Self {
        let now = chrono::Local::now();
        Self::from_chrono_time(now)
    }
}

impl From<i32> for GtfsTime {
    /// Here, value is a number of seconds since midnight
    fn from(value: i32) -> Self {
        let hrs = value / 3600;
        let mins = (value % 3600) / 60;
        let secs = (value % 3600) % 60;
        Self(hrs as u8, mins as u8, secs as u8)
    }
}

impl Into<i32> for GtfsTime {
    /// Returns the GtfsTime as a number of seconds since midnight
    fn into(self) -> i32 {
        (self.0 as i32) * 3600 + (self.1 as i32) * 60 + (self.2 as i32)
    }
}

impl serde::Serialize for GtfsTime {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}:{}:{}", self.0, self.1, self.2);
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for GtfsTime {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut split = s.split(':');
        let h = split.next()
            .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
            .parse::<u8>().map_err(serde::de::Error::custom)?;
        let m = split.next()
            .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
            .parse::<u8>().map_err(serde::de::Error::custom)?;
        let s = split.next()
            .ok_or(serde::de::Error::custom("expected : in GtfsTime"))?
            .parse::<u8>().map_err(serde::de::Error::custom)?;
        Ok(Self(h, m, s))
    }
}