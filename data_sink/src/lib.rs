use influxdb::{Client, InfluxDbWriteable, Query};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum DataSinkError {
    #[error("InfluxDB write error: {0}")]
    WriteError(#[from] influxdb::Error),
}

#[derive(Debug, Serialize)]
struct DataPoint {
    measurement: String,
    fields: HashMap<String, f64>,
}

pub struct DataSink {
    client: Client,
}

impl DataSink {
    pub fn new(base_url: &str, database: &str) -> Self {
        let client = Client::new(base_url, database);
        DataSink { client }
    }

    pub async fn write_data(
        &self,
        measurement: String,
        fields: HashMap<String, f64>,
    ) -> Result<(), DataSinkError> {
        let data_point = DataPoint {
            measurement,
            fields,
        };

        let query = data_point.into_query().unwrap();

        self.client.query(&query).await.map_err(DataSinkError::WriteError)
    }
}

impl InfluxDbWriteable for DataPoint {
    fn into_query(self) -> Result<Query, influxdb::Error> {
        let mut query = Query::write_query(self.measurement, None);

        for (key, value) in self.fields {
            query = query.add_field(&key, value);
        }

        Ok(query)
    }
}
