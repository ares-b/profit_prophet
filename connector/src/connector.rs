use async_trait::async_trait;

use crate::Message;
use crate::http::{HttpClient, HttpClientError};
use crate::influxdb::{InfluxDbClient, InfluxDbClientError};
use crate::postgresql::{PostgresClient, PostgresClientError};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum DataConnectorError {
    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] HttpClientError),
    #[error("InfluxDB client error: {0}")]
    InfluxDbClientError(#[from] InfluxDbClientError),
    #[error("PostgreSQL client error: {0}")]
    PostgresClientError(#[from] PostgresClientError),
    #[error("Other error: {0}")]
    OtherError(String),
}

#[async_trait]
pub trait DataConnector: Send + Sync
{
    async fn write(&self, data: Message) -> Result<Message, DataConnectorError>;
    async fn read(&self, data: Message) -> Result<Message, DataConnectorError>;
}

pub enum Connector {
    Http(HttpClient),
    InfluxDb(InfluxDbClient),
    Postgres(PostgresClient),
}

impl Connector {
    pub async fn write(&self, data: Message) -> Result<Message, DataConnectorError> {
        match self {
            Connector::Http(client) => client.write(data).await,
            Connector::InfluxDb(client) => client.write(data).await,
            Connector::Postgres(client) => client.write(data).await,
        }
    }

    pub async fn read(&self, data: Message) -> Result<Message, DataConnectorError> {
        match self {
            Connector::Http(client) => client.read(data).await,
            Connector::InfluxDb(client) => client.read(data).await,
            Connector::Postgres(client) => client.read(data).await,
        }
    }
}