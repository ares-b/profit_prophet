#[cfg(test)]
mod tests {
    use connector::http::{HttpClient, HttpClientError, HttpRequest};
    use mockito::{mock, Matcher};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Deserialize)]
    struct TestResponse {
        message: String,
    }

    #[tokio::test]
    async fn test_get_request_success() {
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Hello, World!"}"#)
            .create();

        let client = HttpClient::new();
        let url = &mockito::server_url();
        let full_url = format!("{}/test", url);

        let http_request = HttpRequest {
            method: "GET".to_string(),
            url: full_url,
            body: None,
            headers: None,
            query_params: None,
            timeout_duration: None,
        };

        let response = client.request::<(), serde_json::Value>(http_request).await.unwrap();

        assert_eq!(response.status, 200);
        let response_body: TestResponse = serde_json::from_value(response.body.unwrap()).unwrap();
        assert_eq!(response_body.message, "Hello, World!");
    }

    #[tokio::test]
    async fn test_post_request_success() {
        let _m = mock("POST", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Data received!"}"#)
            .match_header("content-type", "application/json")
            .match_body(Matcher::JsonString(r#"{"name":"John Doe","age":30}"#.to_string()))
            .create();

        let client = HttpClient::new();
        let url = &mockito::server_url();
        let full_url = format!("{}/test", url);

        #[derive(Debug, Serialize)]
        struct PostBody {
            name: String,
            age: u8,
        }

        let post_body = PostBody {
            name: "John Doe".to_string(),
            age: 30,
        };

        let http_request = HttpRequest {
            method: "POST".to_string(),
            url: full_url,
            body: Some(serde_json::to_value(post_body).unwrap()),
            headers: Some(HashMap::from([("Content-Type".to_string(), "application/json".to_string())])),
            query_params: None,
            timeout_duration: None,
        };

        let response = client.request(http_request).await.unwrap();

        assert_eq!(response.status, 200);
        let response_body: TestResponse = serde_json::from_value(response.body.unwrap()).unwrap();
        assert_eq!(response_body.message, "Data received!");
    }

    #[tokio::test]
    async fn test_get_request_deserialize_error() {
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("This will cause a deserialize error")
            .create();

        let client = HttpClient::new();
        let url = &mockito::server_url();
        let full_url = format!("{}/test", url);

        let http_request = HttpRequest {
            method: "GET".to_string(),
            url: full_url,
            body: None,
            headers: None,
            query_params: None,
            timeout_duration: None,
        };

        let result = client.request::<(), serde_json::Value>(http_request).await;

        assert!(matches!(result, Err(HttpClientError::DeserializeError(_))));
    }
}
