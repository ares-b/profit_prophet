use std::collections::HashMap;
use crate::connector::{DataConnector, DataConnectorError};
use crate::{Message, Protocol};
use crate::http::{HttpClient, HttpRequest};
use crate::influxdb::protocol::InfluxDbDataPoint;
use async_trait::async_trait;
use crate::influxdb::error::InfluxDbClientError;

#[derive(Clone, Debug)]
pub struct InfluxDbClient {
    http_client: HttpClient,
    base_url: String,
    token: String,
}

impl InfluxDbClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        InfluxDbClient {
            http_client: HttpClient::new(),
            base_url: base_url.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn write_data(
        &self,
        data: InfluxDbDataPoint
    ) -> Result<Option<String>, InfluxDbClientError> {
        let url = format!("{}/api/v2/write", self.base_url);
        let precision = data.infer_precision();
        let data_str = data.to_line_protocol();
        
        let mut query_params = HashMap::new();
        query_params.insert("org".to_string(), data.organization);
        query_params.insert("bucket".to_string(), data.bucket);
        query_params.insert("precision".to_string(), precision);

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Token {}", self.token));
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        let request = HttpRequest {
            method: "POST".to_string(),
            url,
            body: Some(data_str),
            headers: Some(headers),
            query_params: Some(query_params),
            timeout_duration: None,
        };
        match self.http_client.request::<String, String>(request).await {
            Ok(response) => Ok(response.body),
            Err(err) => Err(InfluxDbClientError::HttpClientError(err)),
        }
    }
}

#[async_trait]
impl DataConnector for InfluxDbClient {

    async fn write(&self, data: Message) -> Result<Message, DataConnectorError> {
        let influx_data: InfluxDbDataPoint = match data.payload {
            Protocol::Json(value) => {
                serde_json::from_value(value).map_err(InfluxDbClientError::SerializationError)?
            },
        };
        let result = self.write_data(influx_data).await?;
        let response_payload = serde_json::to_value(result).map_err(InfluxDbClientError::SerializationError)?;

        Ok(Message {
            compression: data.compression,
            payload: Protocol::Json(response_payload),
        })
    }

    async fn read(&self, _data: Message) -> Result<Message, DataConnectorError> {
        Err(DataConnectorError::InfluxDbClientError(
            InfluxDbClientError::UnsupportedValueType("Read operation is not supported".to_string())
        ))
    }
}
