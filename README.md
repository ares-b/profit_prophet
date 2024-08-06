use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::sync::oneshot;
use log::{info, warn};
use serde_json::Value as JsonValue;
use std::fmt::Debug;

use data_source::data_source::DataSource;
use data_sink::data_sink::{DataSink, DataSinkError};
use connector::{Message, Protocol, Compression, DataConnector};
use connector::influxdb::{FieldValue, InfluxDbClient};
use connector::http::{HttpClient, HttpResponse};

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(seconds))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    data_sources: Vec<DataSourceConfig>,
    data_sinks: Vec<DataSinkConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DataSourceConfig {
    connector: ConnectorConfig,
    data_sinks: Vec<String>,
    query: HttpRequestConfig,
    transformations: Vec<TransformationConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ConnectorConfig {
    HttpClient,
    InfluxDbClient { url: String, token: String },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DataSinkConfig {
    connector: ConnectorConfig,
    address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct HttpRequestConfig {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: Option<HashMap<String, JsonValue>>,
    #[serde(deserialize_with = "deserialize_duration")]
    timeout_duration: Duration,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TransformationConfig {
    name: String,
    message: MessageConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct MessageConfig {
    measurement: String,
    bucket: String,
    organization: String,
    fields: Vec<TransformationField>,
    tags: Option<Vec<TransformationField>>,
    timestamp: Option<TransformationField>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "value")]
enum Source {
    Literal(String),
    Field(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TransformationField {
    source: Source,
    target: String,
}

type TransformationFn = fn(&DataSourceConfig, Message) -> Message;

lazy_static::lazy_static! {
    static ref TRANSFORMATION_REGISTRY: Mutex<HashMap<String, TransformationFn>> = Mutex::new(HashMap::new());
}

fn register_transformation(name: &str, func: TransformationFn) {
    TRANSFORMATION_REGISTRY.lock().unwrap().insert(name.to_string(), func);
}

fn get_transformation(name: &str) -> Option<TransformationFn> {
    TRANSFORMATION_REGISTRY.lock().unwrap().get(name).cloned()
}

fn extract_value_from_json(json: &JsonValue, path: &str) -> Option<JsonValue> {
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

fn convert_to_field_value(value: JsonValue) -> FieldValue {
    match value {
        JsonValue::Bool(b) => FieldValue::Bool(b),
        JsonValue::Number(n) => {
            if n.is_i64() {
                FieldValue::I64(n.as_i64().unwrap())
            } else if n.is_u64() {
                FieldValue::I64(n.as_u64().unwrap() as i64)
            } else if n.is_f64(){
                FieldValue::F64(n.as_f64().unwrap())
            } else {
                FieldValue::String(n.to_string())
            }
        }
        JsonValue::String(s) => FieldValue::String(s),
        _ => FieldValue::String(value.to_string()),
    }
}

fn convert_literal_to_field_value(literal: &str) -> FieldValue {
    if let Ok(b) = literal.parse::<bool>() {
        FieldValue::Bool(b)
    } else if let Ok(i) = literal.parse::<i64>() {
        FieldValue::I64(i)
    } else if let Ok(f) = literal.parse::<f64>() {
        FieldValue::F64(f)
    } else {
        FieldValue::String(literal.to_string())
    }
}

fn get_value(source: &Source, response: &JsonValue) -> FieldValue {
    match source {
        Source::Literal(value) => convert_literal_to_field_value(value),
        Source::Field(path) => extract_value_from_json(response, path)
            .map(convert_to_field_value)
            .unwrap_or_else(|| {
                warn!("Field {} not found in response", path);
                FieldValue::String("".to_string())
            }),
    }
}

fn select(config: &DataSourceConfig, message: Message) -> Message {
    info!("Original Message: {:?}", message);

    let Protocol::Json(json_payload) = message.payload;

    info!("Received JSON payload: {}", json_payload);

    let http_response: HttpResponse<JsonValue> = match serde_json::from_value(json_payload.clone()) {
        Ok(res) => res,
        Err(e) => {
            panic!("Failed to deserialize JSON: {}. Payload: {}", e, json_payload);
        }
    };

    let response = serde_json::json!({
        "body": http_response.body,
        "status": http_response.status
    });

    let transformation_config = config.transformations.iter().find(|t| t.name == "select").expect("Transformation config not found");

    let mut final_message = HashMap::new();
    final_message.insert("measurement".to_string(), JsonValue::String(transformation_config.message.measurement.clone()));
    final_message.insert("bucket".to_string(), JsonValue::String(transformation_config.message.bucket.clone()));
    final_message.insert("organization".to_string(), JsonValue::String(transformation_config.message.organization.clone()));

    let mut fields = HashMap::new();
    for field in &transformation_config.message.fields {
        let value = get_value(&field.source, &response);
        info!("Extracted field {} with value: {:?}", field.target, value);
        fields.insert(field.target.clone(), value);
    }
    final_message.insert("fields".to_string(), serde_json::to_value(fields).unwrap());

    let mut tags = HashMap::new();
    if let Some(tag_fields) = &transformation_config.message.tags {
        for tag in tag_fields {
            let value = get_value(&tag.source, &response);
            info!("Extracted tag {} with value: {:?}", tag.target, value);
            if let FieldValue::String(v) = value {
                tags.insert(tag.target.clone(), v);
            } else {
                warn!("Tag values must be strings, got: {:?}", value);
            }
        }
    }
    final_message.insert("tags".to_string(), serde_json::to_value(tags).unwrap());

    let transformed_message = Message {
        compression: Compression::None,
        payload: Protocol::Json(JsonValue::Object(final_message.into_iter().collect())),
    };

    info!("Transformed Message: {:?}", transformed_message);

    transformed_message
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
struct PostBody {
    name: String,
    age: u8,
}

fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(file_path)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

fn create_connector(connector_config: &ConnectorConfig) -> Arc<dyn DataConnector> {
    match connector_config {
        ConnectorConfig::HttpClient => Arc::new(HttpClient::new()),
        ConnectorConfig::InfluxDbClient { url, token } => {
            Arc::new(InfluxDbClient::new(url, token))
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    info!("Loading configuration...");

    let config = load_config("config.yaml")?;

    register_transformation("select", select);

    for data_source_config in config.data_sources {
        let connector = create_connector(&data_source_config.connector);

        let config_clone = data_source_config.clone();

        info!("Starting data source...");

        let data_source = DataSource::new(
            connector.clone(),
            Message {
                compression: Compression::None,
                payload: Protocol::Json(serde_json::to_value(&data_source_config.query).unwrap()),
            },
            Some(data_source_config.query.timeout_duration),
            Some(data_source_config.data_sinks.clone()),
            None,
            Some(Arc::new(move |msg| {
                let transform = get_transformation("select").expect("Transformation function not found");
                transform(&config_clone, msg)
            })),
        );

        let mut ready_receivers = Vec::new();
        let listener_handles: Vec<_> = data_source_config.data_sinks
            .clone()
            .into_iter()
            .map(|addr| {
                let sink_config = config.data_sinks.iter().find(|sink| sink.address == addr).expect("Data sink configuration not found");
                let (ready_tx, ready_rx) = oneshot::channel();
                ready_receivers.push(ready_rx);
                let connector = create_connector(&sink_config.connector);
                tokio::spawn(async move {
                    let mut data_sink = DataSink::new(connector.clone(), &addr);
                    ready_tx.send(()).expect("Failed to send ready signal");
                    if let Err(e) = data_sink.start().await {
                        eprintln!("DataSink failed: {:?}", e);
                    }
                })
            })
            .collect();

        for ready_rx in ready_receivers {
            ready_rx.await.expect("Failed to receive ready signal");
        }

        info!("All listeners are ready. Starting data source...");

        data_source.start().await?;

        for handle in listener_handles {
            handle.await.unwrap();
        }
    }

    Ok(())
}
