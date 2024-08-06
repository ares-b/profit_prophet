#[cfg(test)]
mod tests {
    use connector::influxdb::*;
    use mockito::{mock, server_url, Matcher};
    use std::collections::HashMap;
    use std::time::SystemTime;

    #[tokio::test]
    async fn test_write_data_success() {
        let _m = mock("POST", "/api/v2/write")
            .match_header("Authorization", "Token my-token")
            .match_header("Content-Type", "text/plain")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("org".to_string(), "test_org".to_string()),
                Matcher::UrlEncoded("bucket".to_string(), "test_bucket".to_string()),
                Matcher::UrlEncoded("precision".to_string(), "ns".to_string()),
            ]))
            .with_status(204)
            .create();

        let client = InfluxDbClient::new(&server_url(), "my-token");

        let data_point = InfluxDbDataPoint {
            organization: "test_org".to_string(),
            bucket: "test_bucket".to_string(),
            measurement: "test_measurement".to_string(),
            tags: HashMap::from([("tag1".to_string(), "tag_value".to_string())]),
            fields: HashMap::from([("field1".to_string(), 33.into())]),
            timestamp: SystemTime::now(),
        };

        let result = client.write_data(data_point).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response, None);
    }

    #[tokio::test]
    async fn test_write_data_http_error() {
        let _m = mock("POST", "/api/v2/write")
            .match_header("Authorization", "Token my-token")
            .match_header("Content-Type", "text/plain")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("org".to_string(), "test_org".to_string()),
                Matcher::UrlEncoded("bucket".to_string(), "test_bucket".to_string()),
                Matcher::UrlEncoded("precision".to_string(), "ns".to_string()),
            ]))
            .with_status(500)
            .create();

        let client = InfluxDbClient::new(&server_url(), "my-token");

        let data_point = InfluxDbDataPoint {
            organization: "test_org".to_string(),
            bucket: "test_bucket".to_string(),
            measurement: "test_measurement".to_string(),
            tags: HashMap::from([("tag1".to_string(), "tag_value".to_string())]),
            fields: HashMap::from([("field1".to_string(), 42.32.into())]),
            timestamp: SystemTime::now(),
        };

        let result = client.write_data(data_point).await;

        assert!(result.is_err());
        if let InfluxDbClientError::HttpClientError(_) = result.unwrap_err() {
        } else {
            panic!("Expected HttpClientError");
        }
    }
}
