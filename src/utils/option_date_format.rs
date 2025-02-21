use std::fmt;

use chrono::NaiveDateTime;
use serde::de::{self, Error, Visitor};
use serde::{Deserialize, Serialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(x) = date {
        let s = x.format(FORMAT).to_string();
        return serializer.serialize_str(&s);
    }
    return serializer.serialize_none();
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    struct CustomDateVisitor;

    impl<'de> Visitor<'de> for CustomDateVisitor {
        type Value = Option<NaiveDateTime>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional NaiveDateTime string")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
        {
            let s: &str = Deserialize::deserialize(deserializer)?;
            Ok(Some(NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map_err(Error::custom)?))
        }



    }

    deserializer.deserialize_option(CustomDateVisitor)
}
