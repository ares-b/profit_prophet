use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use crate::connector::{DataConnector, DataConnectorError};
use crate::{Message, Protocol};
use async_trait::async_trait;
use serde_json::Value as JsonValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Failed to deserialize response: {0}")]
    DeserializeError(String),
    #[error("Invalid HTTP method: {0}")]
    InvalidMethodError(String),
    #[error("HTTP error: {0}")]
    HttpError(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpRequest<T> {
    pub method: String,
    pub url: String,
    pub body: Option<T>,
    pub headers: Option<HashMap<String, String>>,
    pub query_params: Option<HashMap<String, String>>,
    pub timeout_duration: Option<Duration>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse<T> {
    pub status: u16,
    pub body: Option<T>,
}

#[derive(Clone, Debug)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }

    pub async fn request<QueryBody: Serialize, ResponseBody: DeserializeOwned>(&self, request: HttpRequest<QueryBody>) -> Result<HttpResponse<ResponseBody>, HttpClientError> {
        let method: Method = request.method.parse().map_err(|_| HttpClientError::InvalidMethodError(request.method.to_string()))?;
        let mut request_builder = self.client.request(method.clone(), &request.url);

        if let Some(ref h) = request.headers {
            for (key, value) in h {
                request_builder = request_builder.header(key, value);
            }
        }

        if let Some(params) = request.query_params {
            request_builder = request_builder.query(&params);
        }

        if let Some(b) = request.body {
            if let Some(content_type) = request.headers.as_ref().and_then(|h| h.get("Content-Type")) {
                match content_type.as_str() {
                    "application/json" => {
                        request_builder = request_builder.json(&b);
                    }
                    "application/x-www-form-urlencoded" => {
                        if let Ok(form_body) = serde_urlencoded::to_string(&b) {
                            request_builder = request_builder.body(form_body);
                        }
                    }
                    "text/plain" => {
                        if let Ok(text_body) = serde_json::to_string(&b) {
                            let text_body = text_body.replace("\\\"", "\"").trim_matches('"').to_string();
                            request_builder = request_builder.body(text_body);
                        }
                    }
                    content_type => {
                        todo!("Unmatched Content-Type {}", content_type);
                    }
                }
            } else {
                request_builder = request_builder.json(&b);
            }
        }

        if let Some(duration) = request.timeout_duration {
            request_builder = request_builder.timeout(duration);
        }

        let request = request_builder.build()?;
        let response = self.client.execute(request).await.map_err(|e| {
            if e.is_timeout() {
                HttpClientError::TimeoutError("Request timed out".to_string())
            } else {
                HttpClientError::RequestError(e)
            }
        })?;
        
        let status = response.status();
        if !status.is_success() {
            return Err(HttpClientError::HttpError(format!("HTTP error: {}", status)));
        }
        
        let response_text = response.text().await.map_err(|e| {
            if e.is_timeout() {
                HttpClientError::TimeoutError("Response timed out".to_string())
            } else {
                HttpClientError::RequestError(e)
            }
        })?;
        
        let body = if response_text.is_empty() {
            None
        } else {
            match serde_json::from_str::<ResponseBody>(&response_text) {
                Ok(parsed_body) => Some(parsed_body),
                Err(_) => return Err(HttpClientError::DeserializeError(response_text.clone())),
            }
        };
        
        Ok(HttpResponse {
            status: status.as_u16(),
            body,
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DataConnector for HttpClient {

    async fn write(&self, data: Message) -> Result<Message, DataConnectorError> {
        let http_request: HttpRequest<JsonValue> = match data.payload {
            Protocol::Json(value) => serde_json::from_value(value).map_err(|e| HttpClientError::DeserializeError(e.to_string()))?,
            // Add cases for other protocols if needed
        };
        let response: HttpResponse<JsonValue> = self.request(http_request).await?;
        let response_payload = serde_json::to_value(response).map_err(|e| HttpClientError::DeserializeError(e.to_string()))?;

        Ok(Message {
            compression: data.compression,
            payload: Protocol::Json(response_payload),
        })
    }

    async fn read(&self, data: Message) -> Result<Message, DataConnectorError> {
        self.write(data).await
    }
}
