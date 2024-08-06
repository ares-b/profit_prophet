use serde::{Deserialize, Serialize};
use std::sync::Arc;
use connector::DataConnector;
use connector::http::HttpClient;
use connector::influxdb::InfluxDbClient;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConnectorConfig {
    HttpClient,
    InfluxDbClient { url: String, token: String },
}

impl ConnectorConfig {
    pub fn create_connector(&self) -> Arc<dyn DataConnector> {
        match self {
            ConnectorConfig::HttpClient => Arc::new(HttpClient::new()),
            ConnectorConfig::InfluxDbClient { url, token } => Arc::new(InfluxDbClient::new(url, token)),
        }
    }
}
