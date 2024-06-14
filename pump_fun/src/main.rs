#[tokio::main]
async fn main() {
    use connector::http::{HttpClient, HttpClientError};
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::time::Duration;
    use data_sink::DataSink;

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
        Ok(response) => {
            println!("GET response: {:?}", response);
            let data_sink = DataSink::new("http://localhost:8086", "db_name");

            let mut fields = HashMap::new();
            fields.insert("response_time".to_string(), 1.0);

            match data_sink.write_data("get_request".to_string(), fields).await {
                Ok(_) => println!("Data written to InfluxDB successfully"),
                Err(e) => eprintln!("Failed to write data to InfluxDB: {:?}", e),
            }
        },
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
        Ok(response) => {
            println!("POST response: {:?}", response);
            let data_sink = DataSink::new("http://localhost:8086", "db_name");

            let mut fields = HashMap::new();
            fields.insert("response_time".to_string(), 1.0);

            match data_sink.write_data("post_request".to_string(), fields).await {
                Ok(_) => println!("Data written to InfluxDB successfully"),
                Err(e) => eprintln!("Failed to write data to InfluxDB: {:?}", e),
            }
        },
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
}