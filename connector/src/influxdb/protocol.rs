use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{self, Visitor, MapAccess};

#[derive(Serialize, Debug)]
pub enum FieldValue {
    Bool(bool),
    F64(f64),
    I64(i64),
    String(String),
}

impl From<bool> for FieldValue {
    fn from(other: bool) -> Self {
        Self::Bool(other)
    }
}

impl From<f64> for FieldValue {
    fn from(other: f64) -> Self {
        Self::F64(other)
    }
}

impl From<i64> for FieldValue {
    fn from(other: i64) -> Self {
        Self::I64(other)
    }
}

impl From<&str> for FieldValue {
    fn from(other: &str) -> Self {
        Self::String(other.into())
    }
}

impl<'de> Deserialize<'de> for FieldValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldValueVisitor;

        impl<'de> Visitor<'de> for FieldValueVisitor {
            type Value = FieldValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid FieldValue")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FieldValue::Bool(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FieldValue::I64(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= i64::MAX as u64 {
                    Ok(FieldValue::I64(value as i64))
                } else {
                    Err(de::Error::custom("u64 value is too large to fit in i64"))
                }
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FieldValue::F64(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FieldValue::String(value.to_string()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FieldValue::String(value))
            }
        }

        deserializer.deserialize_any(FieldValueVisitor)
    }
}

#[derive(Serialize, Debug)]
pub struct InfluxDbDataPoint {
    pub organization: String,
    pub bucket: String,
    pub measurement: String,
    pub tags: HashMap<String, String>,
    pub fields: HashMap<String, FieldValue>,
    pub timestamp: SystemTime,
}

impl InfluxDbDataPoint {
    pub fn to_line_protocol(&self) -> String {
        let mut line: String = self.measurement.to_string();

        let tags = self.tags.iter().map(
            |(key, value)| format!("{key}=\"{value}\"")
        ).collect::<Vec<_>>().join(",");
        line.push_str(&format!(",{}", tags));

        let fields = self.fields.iter().map(
            |(key, value)| match value {
                FieldValue::String(s) => format!("{}=\"{}\"", key, s),
                FieldValue::F64(n) => format!("{}={}", key, n),
                FieldValue::I64(n) => format!("{}={}", key, n),
                FieldValue::Bool(b) => format!("{}={}", key, b),
            }
        ).collect::<Vec<_>>().join(",");
        line.push_str(&format!(" {}", fields));
        
        let duration = self.timestamp.duration_since(UNIX_EPOCH).unwrap();
        line.push_str(&format!(" {}", match self.infer_precision().as_str() {
            "s" => duration.as_secs().to_string(),
            "ms" => duration.as_millis().to_string(),
            "us" => duration.as_micros().to_string(),
            _ => duration.as_nanos().to_string(),
        }));

        line
    }

    pub fn infer_precision(&self) -> String {
        let duration = self.timestamp.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let nanos = duration.as_nanos();
        if nanos % 1_000_000_000 == 0 {
            "s".to_string()
        } else if nanos % 1_000_000 == 0 {
            "ms".to_string()
        } else if nanos % 1_000 == 0 {
            "us".to_string()
        } else {
            "ns".to_string()
        }
    }
}

impl<'de> Deserialize<'de> for InfluxDbDataPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Organization,
            Bucket,
            Measurement,
            Tags,
            Fields,
            Timestamp,
        }

        struct InfluxDbDataPointVisitor;

        impl<'de> Visitor<'de> for InfluxDbDataPointVisitor {
            type Value = InfluxDbDataPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct InfluxDbDataPoint")
            }

            fn visit_map<V>(self, mut map: V) -> Result<InfluxDbDataPoint, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut organization = None;
                let mut bucket = None;
                let mut measurement = None;
                let mut tags = None;
                let mut fields = None;
                let mut timestamp = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Organization => {
                            if organization.is_some() {
                                return Err(de::Error::duplicate_field("organization"));
                            }
                            organization = Some(map.next_value()?);
                        }
                        Field::Bucket => {
                            if bucket.is_some() {
                                return Err(de::Error::duplicate_field("bucket"));
                            }
                            bucket = Some(map.next_value()?);
                        }
                        Field::Measurement => {
                            if measurement.is_some() {
                                return Err(de::Error::duplicate_field("measurement"));
                            }
                            measurement = Some(map.next_value()?);
                        }
                        Field::Tags => {
                            if tags.is_some() {
                                return Err(de::Error::duplicate_field("tags"));
                            }
                            tags = Some(map.next_value()?);
                        }
                        Field::Fields => {
                            if fields.is_some() {
                                return Err(de::Error::duplicate_field("fields"));
                            }
                            let fields_map: HashMap<String, FieldValue> = map.next_value()?;
                            fields = Some(fields_map);
                        }
                        Field::Timestamp => {
                            if timestamp.is_some() {
                                return Err(de::Error::duplicate_field("timestamp"));
                            }
                            let ts: Option<u64> = map.next_value()?;
                            timestamp = Some(ts.map_or_else(SystemTime::now, |secs| UNIX_EPOCH + Duration::from_secs(secs)));
                        }
                    }
                }

                let organization = organization.ok_or_else(|| de::Error::missing_field("organization"))?;
                let bucket = bucket.ok_or_else(|| de::Error::missing_field("bucket"))?;
                let measurement = measurement.ok_or_else(|| de::Error::missing_field("measurement"))?;
                let tags = tags.unwrap_or_default();
                let fields = fields.unwrap_or_default();
                let timestamp = timestamp.unwrap_or_else(SystemTime::now);

                Ok(InfluxDbDataPoint {
                    organization,
                    bucket,
                    measurement,
                    tags,
                    fields,
                    timestamp,
                })
            }
        }

        const FIELDS: &[&str] = &["organization", "bucket", "measurement", "tags", "fields", "timestamp"];
        deserializer.deserialize_struct("InfluxDbDataPoint", FIELDS, InfluxDbDataPointVisitor)
    }
}