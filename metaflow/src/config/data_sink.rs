use data_sink::data_sink::DataSink;
use serde::{Deserialize, Serialize};
use crate::config::connector::ConnectorConfig;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataSinkConfig {
    pub name: String,
    pub connector: ConnectorConfig,
    pub address: String,
}

impl DataSinkConfig {
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let connector = self.connector.create_connector();
        let mut data_sink = DataSink::new(connector.clone(), &self.address);
        data_sink.start().await?;
        Ok(())
    }
}
