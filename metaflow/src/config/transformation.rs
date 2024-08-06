use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use log::error;
use connector::{Message, Compression, Protocol};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransformationConfig {
    pub name: String,
    pub message: TransformationMessage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransformationMessage {
    pub protocol: Option<String>,
    pub compression: Option<Compression>,
    pub data: HashMap<String, TransformationSourceConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum TransformationSourceConfig {
    Literal(String),
    Field(String),
    Computed(String),
    Object(HashMap<String, TransformationSourceConfig>),
}

type TransformationRegisty = Mutex<HashMap<String, fn(&TransformationConfig, Message) -> Message>>;

lazy_static::lazy_static! {
    static ref TRANSFORMATION_REGISTRY: TransformationRegisty = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
pub fn register(name: &str, func: fn(&TransformationConfig, Message) -> Message) {
    TRANSFORMATION_REGISTRY.lock().unwrap().insert(name.to_string(), func);
}

pub fn get(name: &str) -> Option<fn(&TransformationConfig, Message) -> Message> {
    TRANSFORMATION_REGISTRY.lock().unwrap().get(name).cloned()
}

#[allow(dead_code)]
pub fn get_value(source: &TransformationSourceConfig, protocol: &Protocol) -> Protocol {
    match source {
        TransformationSourceConfig::Literal(value) => match protocol {
            Protocol::Json(_) => Protocol::Json(serde_json::Value::String(value.clone())),
        },
        TransformationSourceConfig::Field(path) => match protocol {
            Protocol::Json(json) => extract_value_from_json(json, path)
                .map(Protocol::Json)
                .unwrap_or_else(|| {
                    error!("Field {} not found in response", path);
                    Protocol::Json(serde_json::Value::String("".to_string()))
                }),
        },
        TransformationSourceConfig::Computed(value) => match protocol {
            Protocol::Json(_) => {
                if value == "current_timestamp" {
                    Protocol::Json(serde_json::Value::Number(serde_json::Number::from(
                        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                    )))
                } else {
                    error!("Computed value {} is not supported", value);
                    Protocol::Json(serde_json::Value::String("".to_string()))
                }
            },
        },
        TransformationSourceConfig::Object(fields) => match protocol {
            Protocol::Json(_) => {
                let mut field_data = serde_json::Map::new();
                for (field_key, field) in fields {
                    let value = get_value(field, protocol);
                    let Protocol::Json(json_value) = value;
                    field_data.insert(field_key.clone(), json_value);
                }
                Protocol::Json(serde_json::Value::Object(field_data))
            },
        },
    }
}

#[allow(dead_code)]
fn extract_value_from_json(json: &serde_json::Value, path: &str) -> Option<serde_json::Value> {
    let keys: Vec<&str> = path.split('.').collect();
    let mut current = json;
    for key in keys {
        current = match current.get(key) {
            Some(value) => value,
            None => return None,
        };
    }
    Some(current.clone())
}
