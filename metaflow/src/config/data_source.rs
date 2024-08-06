use std::sync::Arc;
use std::collections::HashMap;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use log::{info, error};
use connector::{Compression, Message, Protocol};
use data_source::data_source::DataSource;
use crate::config::connector::ConnectorConfig;
use crate::config::transformation::TransformationConfig;
use crate::config::transformation;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataSourceConfig {
    pub name: String,
    pub connector: ConnectorConfig,
    pub data_sinks: Vec<String>,
    pub query: HttpRequestConfig,
    pub transformation: TransformationConfig,
}

impl DataSourceConfig {
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let connector = self.connector.create_connector();

        let transformation_config = self.transformation.clone();

        info!("Starting data source...");

        let transformation_fn: Option<Arc<dyn Fn(Message) -> Message + Send + Sync>> = match transformation::get(&self.transformation.name) {
            Some(transformation_fn) => Some(Arc::new(
                move |msg| {
                    transformation_fn(&transformation_config, msg)
                }
            )),
            None => {
                error!("Transformation function {} not found", &self.transformation.name);
                None
            }
        };

        let data_source = DataSource::new(
            connector.clone(),
            Message {
                compression: Compression::None,
                payload: Protocol::Json(serde_json::to_value(&self.query).unwrap()),
            },
            Some(self.query.timeout_duration),
            Some(self.data_sinks.clone()),
            None,
            transformation_fn,
        );

        data_source.start().await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HttpRequestConfig {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Option<HashMap<String, serde_json::Value>>,
    #[serde(deserialize_with = "deserialize_duration")]
    pub timeout_duration: std::time::Duration,
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_duration(&s).map_err(serde::de::Error::custom)
}

fn parse_duration(s: &str) -> Result<std::time::Duration, String> {
    let unit = &s[s.len() - 1..];
    let value = &s[..s.len() - 1];
    let number = u64::from_str(value).map_err(|_| "Invalid number")?;
    match unit {
        "s" => Ok(std::time::Duration::from_secs(number)),
        "m" => Ok(std::time::Duration::from_secs(number * 60)),
        "h" => Ok(std::time::Duration::from_secs(number * 3600)),
        _ => Err("Invalid unit".to_string()),
    }
}
