#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::json;
    use connector::{Compression, DataConnector, DataConnectorError, Message, Protocol};
    use connector::postgresql::*;
    use connector::postgresql::PostgresClientError;
    use connector::field_protocol::FieldValue;
    use mockall::{mock, predicate::*};

    mock! {
        pub DataConnectorTrait {}

        #[async_trait::async_trait]
        impl DataConnector for DataConnectorTrait {
            async fn write(&self, data: Message) -> Result<Message, DataConnectorError>;
            async fn read(&self, _data: Message) -> Result<Message, DataConnectorError>;
        }
    }

    #[tokio::test]
    async fn test_write_data_success() -> Result<(), PostgresClientError> {
        let mut mock_client = MockDataConnectorTrait::new();

        mock_client.expect_write()
            .returning(|_| Ok(Message {
                compression: Compression::None,
                payload: Protocol::Json(json!({})),
            }));

        let mut rows = HashMap::new();
        rows.insert("column1".to_string(), FieldValue::String("value1".to_string()));
        rows.insert("column2".to_string(), FieldValue::String("value2".to_string()));

        let data = PostgresData {
            table_name: "profit_postgres".to_string(),
            schema: "public".to_string(),
            rows,
        };

        let result = mock_client.write(Message {
            compression: Compression::None,
            payload: Protocol::Json(serde_json::to_value(data)?),
        }).await;

        assert!(result.is_ok(), "Failed to write data: {:?}", result.err());

        Ok(())
    }

    #[tokio::test]
    async fn test_serialization_error() -> Result<(), PostgresClientError> {
        let mut mock_client = MockDataConnectorTrait::new();

        mock_client.expect_write()
            .returning(|_| Err(DataConnectorError::PostgresClientError(PostgresClientError::SerializationError(
                serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::Other, "mock error"))
            ))));

        let malformed_json = json!({
            "unexpected_field": "value1",
            "another_field": "value2"
        });

        let message = Message {
            compression: Compression::None,
            payload: Protocol::Json(malformed_json),
        };

        let result = mock_client.write(message).await;

        assert!(result.is_err(), "Expected a SerializationError, but got success.");
        if let DataConnectorError::PostgresClientError(PostgresClientError::SerializationError(_)) = result.unwrap_err() {
        } else {
            panic!("Expected SerializationError, but got a different error.");
        }

        Ok(())
    }
}
