use serde::{Deserialize, Serialize};
use std::sync::Arc;
use connector::DataConnector;
use connector::http::HttpClient;
use connector::influxdb::InfluxDbClient;
use connector::postgresql::PostgresClient;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConnectorConfig {
    HttpClient,
    InfluxDbClient { url: String, token: String },
    PostgresClient {
        host: String,
        port: u16,
        user: String,
        password: String,
        dbname: String,
    },
}

impl ConnectorConfig {
    pub fn create_connector(&self) -> Arc<dyn DataConnector> {
        match self {
            ConnectorConfig::HttpClient => Arc::new(HttpClient::new()),
            ConnectorConfig::InfluxDbClient { url, token } => Arc::new(InfluxDbClient::new(url, token)),
            ConnectorConfig::PostgresClient { host, port, user, password, dbname } => { // TODO : to update
                let client = tokio::runtime::Handle::current().block_on(PostgresClient::new(
                    &format!("host={} port={} user={} password={} dbname={}", host, port, user, password, dbname)
                )).unwrap();
                Arc::new(client)
            },
        }
    }
}
