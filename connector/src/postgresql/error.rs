use thiserror::Error;
use tokio_postgres::Error as PostgresError;

#[derive(Error, Debug)]
pub enum PostgresClientError {
    #[error("Postgres error: {0}")]
    PostgresError(#[from] PostgresError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    OtherError(String),
}
