use async_trait::async_trait;
use connector::http::HttpRequest;
use connector::{Message, Protocol, Compression};
use connector::{DataConnector, DataConnectorError};
use data_source::data_source::DataSource;
use mockito::{mock, Matcher};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;
use tokio::io::AsyncReadExt;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
struct TestResponse {
    message: String,
}

#[derive(Debug)]
pub struct MockConnectorError(String);

impl fmt::Display for MockConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MockConnectorError: {}", self.0)
    }
}

impl std::error::Error for MockConnectorError {}

#[derive(Debug, Clone)]
struct MockConnector;

#[async_trait]
impl DataConnector for MockConnector {
    async fn write(&self, _data: Message) -> Result<Message, DataConnectorError> {
        unimplemented!()
    }

    async fn read(&self, _data: Message) -> Result<Message, DataConnectorError> {
        Ok(Message {
            compression: Compression::None,
            payload: Protocol::Json(serde_json::json!({
                "status": 200,
                "body": {
                    "message": "Hello, World!"
                }
            })),
        })
    }
}

#[derive(Debug, Serialize, Clone)]
struct PostBody {
    name: String,
    age: u8,
}

async fn start_data_source(data_source: DataSource) {
    data_source.start().await.unwrap();
}

#[tokio::test]
async fn test_data_source_start_get_request_success() {
    let _m = mock("GET", "/test")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Hello, World!"}"#)
        .create();

    let url = format!("{}/test", &mockito::server_url());
    let method = "GET".to_string();

    let connector = Arc::new(MockConnector);

    let query = HttpRequest::<()> {
        method,
        url,
        body: None,
        headers: None,
        query_params: None,
        timeout_duration: None,
    };

    let data_source = DataSource::new(
        connector,
        Message {
            compression: Compression::None,
            payload: Protocol::Json(serde_json::to_value(query).unwrap()),
        },
        Some(Duration::from_secs(1)),
        None,
        None,
        None,
    );

    let data_source_handle = tokio::spawn(start_data_source(data_source));

    // Wait for a few intervals to pass
    tokio::time::sleep(Duration::from_secs(3)).await;
    data_source_handle.abort();
}

#[tokio::test]
async fn test_data_source_start_post_request_success() {
    let _m = mock("POST", "/test")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Data received!"}"#)
        .match_header("content-type", "application/json")
        .match_body(Matcher::JsonString(r#"{"name":"John Doe","age":30}"#.to_string()))
        .create();

    let post_body = PostBody {
        name: "John Doe".to_string(),
        age: 30,
    };

    let url = format!("{}/test", &mockito::server_url());
    let method = "POST".to_string();

    let headers = Some(HashMap::from([("Content-Type".to_string(), "application/json".to_string())]));

    let connector = Arc::new(MockConnector);

    let query = HttpRequest {
        method,
        url,
        body: Some(serde_json::to_value(post_body.clone()).unwrap()),
        headers,
        query_params: None,
        timeout_duration: None,
    };

    let data_source = DataSource::new(
        connector,
        Message {
            compression: Compression::None,
            payload: Protocol::Json(serde_json::to_value(query).unwrap()),
        },
        Some(Duration::from_secs(1)),
        None,
        None,
        None,
    );

    let data_source_handle = tokio::spawn(start_data_source(data_source));

    // Wait for a few intervals to pass
    tokio::time::sleep(Duration::from_secs(3)).await;
    data_source_handle.abort();
}

#[tokio::test]
async fn test_data_source_with_tcp_server() {
    // Mock the HTTP server
    let _m = mock("GET", "/test")
        .match_query(Matcher::UrlEncoded("param1".into(), "value1".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Hello, World!"}"#)
        .create();

    // Mock the TCP server using tokio::net::TcpListener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_addr = listener.local_addr().unwrap();

    let url = format!("{}/test", &mockito::server_url());
    let method = "GET".to_string();

    let query_params = Some(HashMap::from([("param1".to_string(), "value1".to_string())]));

    let connector = Arc::new(MockConnector);

    let query = HttpRequest::<()> {
        method,
        url,
        body: None,
        headers: None,
        query_params,
        timeout_duration: None,
    };

    let data_source = DataSource::new(
        connector,
        Message {
            compression: Compression::None,
            payload: Protocol::Json(serde_json::to_value(query).unwrap()),
        },
        Some(Duration::from_secs(1)),
        Some(vec![local_addr.to_string()]),
        None,
        None,
    );

    let data_source_handle = tokio::spawn(start_data_source(data_source));

    // Accept the TCP connection and verify the received message
    let tcp_handle = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        let received = String::from_utf8_lossy(&buf[..n]);
        assert!(received.contains("Hello, World!"));
    });

    // Wait briefly to allow the interaction to occur
    tokio::time::sleep(Duration::from_secs(3)).await;
    data_source_handle.abort();
    tcp_handle.await.unwrap();
}

#[tokio::test]
async fn test_data_source_with_interval() {
    let _m = mock("GET", "/test")
        .match_query(Matcher::UrlEncoded("param1".into(), "value1".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Hello, World!"}"#)
        .create();

    let url = format!("{}/test", &mockito::server_url());
    let method = "GET".to_string();

    let query_params = Some(HashMap::from([("param1".to_string(), "value1".to_string())]));

    let connector = Arc::new(MockConnector);

    let query = HttpRequest::<()> {
        method,
        url,
        body: None,
        headers: None,
        query_params,
        timeout_duration: None,
    };

    let data_source = DataSource::new(
        connector,
        Message {
            compression: Compression::None,
            payload: Protocol::Json(serde_json::to_value(query).unwrap()),
        },
        Some(Duration::from_secs(1)),
        None,
        None,
        None,
    );

    let data_source_handle = tokio::spawn(start_data_source(data_source));

    // Wait for a few intervals to pass
    tokio::time::sleep(Duration::from_secs(3)).await;
    data_source_handle.abort();
}
