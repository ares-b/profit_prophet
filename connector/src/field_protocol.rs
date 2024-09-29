use std::fmt;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

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

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldValue::Bool(b) => write!(f, "{}", b),
            FieldValue::F64(n) => write!(f, "{}", n),
            FieldValue::I64(n) => write!(f, "{}", n),
            FieldValue::String(s) => write!(f, "{}", s),
        }
    }
}
