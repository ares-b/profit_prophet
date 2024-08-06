use thiserror::Error;
use crate::http::HttpClientError;

#[derive(Error, Debug)]
pub enum InfluxDbClientError {
    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] HttpClientError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Unsupported tag or field value type")]
    UnsupportedValueType(String),
    #[error("Time error: {0}")]
    TimeError(#[from] std::time::SystemTimeError),
}