use influxdb::{Client, Query, Timestamp};
use influxdb::InfluxDbWriteable;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfluxDbClientError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to deserialize response: {0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}

#[derive(Clone)]
pub struct InfluxDbClient {
    client: Client,
}

impl InfluxDbClient {
    pub fn new(database_url: &str, database_name: &str) -> Self {
        InfluxDbClient {
            client: Client::new(database_url, database_name),
        }
    }

    pub async fn write_data<T: InfluxDbWriteable>(&self, data: T) -> Result<(), InfluxDbClientError> {
        self.client.query(&data.into_query("measurement")).await?;
        Ok(())
    }

    pub async fn read_data<T: DeserializeOwned>(&self, query: &str) -> Result<Vec<T>, InfluxDbClientError> {
        let query = Query::raw_read_query(query);
        let result = self.client.query(query).await?;

        let serialized = serde_json::to_string(&result)?;
        let deserialized: Vec<T> = serde_json::from_str(&serialized)?;
        
        Ok(deserialized)
    }
}

impl Default for InfluxDbClient {
    fn default() -> Self {
        Self::new("http://localhost:8086", "default_database")
    }
}
