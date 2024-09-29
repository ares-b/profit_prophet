use crate::connector::{DataConnector, DataConnectorError};
use crate::postgresql::error::PostgresClientError;
use crate::{Message, Protocol};
use async_trait::async_trait;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use crate::postgresql::PostgresData;

#[derive(Clone)]
pub struct PostgresClient {
    client: Arc<Client>,
}

impl PostgresClient {
    pub async fn new(connection_string: &str) -> Result<Self, PostgresClientError> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(PostgresClient {
            client: Arc::new(client),
        })
    }

    pub async fn write_data(&self, data: PostgresData) -> Result<(), PostgresClientError> {
        let columns: Vec<&str> = data.rows.keys().map(|k| k.as_str()).collect();

        let value_strings: Vec<String> = data.rows.values().map(|v| v.to_string()).collect();

        let values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = value_strings
            .iter()
            .map(|s| s as &(dyn tokio_postgres::types::ToSql + Sync))
            .collect();

        let mut query = format!(
            "INSERT INTO {}.{} ({}) VALUES (",
            data.schema, data.table_name, columns.join(", ")
        );

        let placeholders: Vec<String> = (1..=values.len()).map(|i| format!("${}", i)).collect();
        query.push_str(&placeholders.join(", "));
        query.push(')');

        let stmt = self.client.prepare(&query).await?;
        self.client.execute(&stmt, &values).await?;

        Ok(())
    }

    pub async fn execute(&self, query: &str) -> Result<(), PostgresClientError> {
        self.client.batch_execute(query).await.map_err(PostgresClientError::from)
    }
}

#[async_trait]
impl DataConnector for PostgresClient {

    async fn write(&self, data: Message) -> Result<Message, DataConnectorError> {
        let postgres_data: PostgresData = match data.payload {
            Protocol::Json(ref value) => {
                serde_json::from_value(value.clone()).map_err(PostgresClientError::SerializationError)?
            },
        };

        self.write_data(postgres_data).await?;
        Ok(data)
    }

    async fn read(&self, _data: Message) -> Result<Message, DataConnectorError> {
        Err(DataConnectorError::OtherError("Read operation is not supported".to_string()))
    }
}
