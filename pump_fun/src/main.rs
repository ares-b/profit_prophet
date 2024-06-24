#[tokio::main]
async fn main() {
    use connector::http::{HttpClient, HttpClientError};
    use connector::influxdb::{InfluxDbClient, InfluxDbClientError};
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::time::Duration;
    use influxdb::InfluxDbWriteable;
    use influxdb::{Timestamp, Query};

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct EchoResponse {
        method: String,
        protocol: String,
        host: String,
        path: String,
        ip: String,
        headers: HashMap<String, String>,
        #[serde(rename = "parsedQueryParams")]
        parsed_query_params: HashMap<String, String>,
        #[serde(rename = "parsedBody")]
        parsed_body: Option<HashMap<String, serde_json::Value>>,
        #[serde(rename = "rawBody")]
        raw_body: Option<String>,
        warnings: Option<Vec<String>>,
    }

    #[derive(Debug, serde::Serialize)]
    struct PostBody {
        name: String,
        age: u8,
    }

    let client = HttpClient::new();

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let mut query_params = HashMap::new();
    query_params.insert("key".to_string(), "value".to_string());

    let post_body = PostBody {
        name: "John Doe".to_string(),
        age: 30,
    };

    match client
        .request::<EchoResponse, _>(
            "GET",
            "https://echo.free.beeceptor.com",
            None::<&()>,
            Some(headers.clone()),
            Some(query_params.clone()),
            Some(Duration::from_secs(10)),
        )
        .await
    {
        Ok(response) => println!("GET response: {:?}", response),
        Err(e) => match e {
            HttpClientError::RequestError(reqwest_error) => {
                eprintln!("Request error: {:?}", reqwest_error);
            }
            HttpClientError::DeserializeError(response_body) => {
                eprintln!("Failed to deserialize response: {:?}", response_body);
            }
            HttpClientError::TimeoutError(timeout_error) => {
                eprintln!("Timeout error: {:?}", timeout_error);
            }
            HttpClientError::InvalidMethodError(method_error) => {
                eprintln!("Invalid method error: {:?}", method_error);
            }
        },
    }

    match client
        .request::<EchoResponse, _>(
            "POST",
            "https://echo.free.beeceptor.com",
            Some(&post_body),
            Some(headers),
            Some(query_params),
            None,
        )
        .await
    {
        Ok(response) => println!("POST response: {:?}", response),
        Err(e) => match e {
            HttpClientError::RequestError(reqwest_error) => {
                eprintln!("Request error: {:?}", reqwest_error);
            }
            HttpClientError::DeserializeError(response_body) => {
                eprintln!("Failed to deserialize response: {:?}", response_body);
            }
            HttpClientError::TimeoutError(timeout_error) => {
                eprintln!("Timeout error: {:?}", timeout_error);
            }
            HttpClientError::InvalidMethodError(method_error) => {
                eprintln!("Invalid method error: {:?}", method_error);
            }
        },
    }

    let influxdb_client = InfluxDbClient::new("http://localhost:8086", "example_db");

    let temperature = Temperature {
        time: Timestamp::Now,
        value: 22.5,
    };

    match influxdb_client.write_data(temperature).await {
        Ok(_) => println!("Data written to InfluxDB successfully."),
        Err(e) => match e {
            InfluxDbClientError::RequestError(reqwest_error) => {
                log_error(&format!("InfluxDB request error: {:?}", reqwest_error));
            }
            InfluxDbClientError::DeserializeError(serde_error) => {
                log_error(&format!("InfluxDB deserialization error: {:?}", serde_error));
            }
            InfluxDbClientError::InvalidQuery(query_error) => {
                log_error(&format!("InfluxDB query error: {:?}", query_error));
            }
        },
    }

    let read_query = "SELECT * FROM temperature";

    match influxdb_client.read_data::<Temperature>(read_query).await {
        Ok(data) => println!("Data read from InfluxDB: {:?}", data),
        Err(e) => match e {
            InfluxDbClientError::RequestError(reqwest_error) => {
                log_error(&format!("InfluxDB request error: {:?}", reqwest_error));
            }
            InfluxDbClientError::DeserializeError(serde_error) => {
                log_error(&format!("InfluxDB deserialization error: {:?}", serde_error));
            }
            InfluxDbClientError::InvalidQuery(query_error) => {
                log_error(&format!("InfluxDB query error: {:?}", query_error));
            }
        },
    }
}