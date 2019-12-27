// Useful deserialization functions

use crate::*;
use chrono::NaiveDate;
use reqwest::StatusCode;
use serde::de::{self, Deserializer, MapAccess, Unexpected, Visitor};
use serde::Deserialize;
use std::convert::TryInto;
use std::fmt::{self, Formatter};
use std::time::{Duration, Instant};

pub(crate) fn instant_from_seconds<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: Deserializer<'de>,
{
    struct Expires;

    impl<'de> Visitor<'de> for Expires {
        type Value = Instant;
        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "number of seconds until the token expires")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Instant::now() + Duration::from_secs(v))
        }
    }

    deserializer.deserialize_u64(Expires)
}

pub(crate) fn duration_from_secs<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    struct Secs;

    impl<'de> Visitor<'de> for Secs {
        type Value = Duration;
        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "seconds")
        }
        fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Duration::from_secs_f64(v))
        }
    }

    deserializer.deserialize_f64(Secs)
}

pub(crate) fn duration_from_millis<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    struct Millis;

    impl<'de> Visitor<'de> for Millis {
        type Value = Duration;
        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "milliseconds")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Duration::from_millis(v))
        }
    }

    deserializer.deserialize_u64(Millis)
}

pub(crate) fn duration_from_millis_option<'de, D>(
    deserializer: D,
) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "duration_from_millis")] Duration);

    let o = Option::deserialize(deserializer)?;
    Ok(o.map(|Wrapper(val)| val))
}

pub(crate) fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    struct Code;

    impl<'de> Visitor<'de> for Code {
        type Value = StatusCode;
        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "an HTTP Status code")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            StatusCode::from_u16(v.try_into().map_err(E::custom)?).map_err(E::custom)
        }
    }

    deserializer.deserialize_u16(Code)
}

pub(crate) fn deserialize_disallows<'de, D>(deserializer: D) -> Result<Vec<Disallow>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DisallowsVisitor;

    impl<'de> Visitor<'de> for DisallowsVisitor {
        type Value = Vec<Disallow>;
        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "a disallows map")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut v = Vec::with_capacity(10);

            while let Some((key, val)) = map.next_entry::<Disallow, Option<bool>>()? {
                if val == Some(true) {
                    v.push(key);
                }
            }

            Ok(v)
        }
    }

    deserializer.deserialize_map(DisallowsVisitor)
}

pub(crate) fn uri_to_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct UriVisitor;

    impl<'de> Visitor<'de> for UriVisitor {
        type Value = String;
        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "a Spotify URI")
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            let mut parts = v.split(':');

            let first = parts.next().ok_or_else(|| E::missing_field("spotify"))?;
            if first != "spotify" {
                return Err(E::invalid_value(de::Unexpected::Str(first), &self));
            }

            parts.next().ok_or_else(|| E::missing_field("type"))?;

            let third = parts.next().ok_or_else(|| E::missing_field("id"))?;

            if let Some(val) = parts.next() {
                return Err(E::unknown_field(val, &[]));
            }

            Ok(third.to_owned())
        }
    }

    deserializer.deserialize_str(UriVisitor)
}

pub(crate) fn from_arbitrary_date_precision<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateVisitor;

    impl<'de> Visitor<'de> for DateVisitor {
        type Value = NaiveDate;
        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "a date")
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            let mut parts = v.splitn(3, '-');

            let year: i32 = parts.next().unwrap().parse().map_err(E::custom)?;
            let month: u32 = match parts.next() {
                Some(val) => val.parse().map_err(E::custom)?,
                None => 1,
            };
            let day: u32 = match parts.next() {
                Some(val) => val.parse().map_err(E::custom)?,
                None => 1,
            };

            Ok(NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| E::invalid_value(Unexpected::Str(v), &self))?)
        }
    }

    deserializer.deserialize_str(DateVisitor)
}
