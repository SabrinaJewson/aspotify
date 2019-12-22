use reqwest::StatusCode;
use serde::de::{self, Deserializer, Visitor};
use std::convert::TryInto;
use std::fmt;
use std::time::{Duration, Instant};

pub(crate) fn from_seconds<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: Deserializer<'de>,
{
    struct Expires;

    impl<'de> Visitor<'de> for Expires {
        type Value = Instant;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Number of seconds until the token expires")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Instant::now() + Duration::from_secs(v))
        }
    }

    deserializer.deserialize_u64(Expires)
}

pub(crate) fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    struct Code;

    impl<'de> Visitor<'de> for Code {
        type Value = StatusCode;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "HTTP Status code")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            StatusCode::from_u16(v.try_into().map_err(E::custom)?).map_err(E::custom)
        }
    }

    deserializer.deserialize_u16(Code)
}
