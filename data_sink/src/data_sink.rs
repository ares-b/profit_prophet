use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use connector::{DataConnector, Message};
use connector::tcp::server::Server;

#[derive(Debug, thiserror::Error)]
pub enum DataSinkError {
    #[error("Connector error: {0}")]
    ConnectorError(String),
    #[error("TCP error: {0}")]
    TcpError(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] serde_json::Error),
}

pub struct DataSink
{
    connector: Arc<dyn DataConnector>,
    address: String,
    cancellation_token: CancellationToken,
}

impl DataSink
{
    pub fn new(connector: Arc<dyn DataConnector>, address: &str) -> Self {
        DataSink {
            connector,
            address: address.to_string(),
            cancellation_token: CancellationToken::new()
        }
    }

    pub fn stop(&self) {
        self.cancellation_token.cancel()
    }

    pub async fn start(&mut self) -> Result<(), DataSinkError> {
        let (tx, mut rx) = mpsc::channel::<Message>(100);
        let addr = self.address.clone();
        tokio::spawn(async move {
            let server = Server::new(&addr, tx);
            if let Err(e) = server.start().await {
                eprintln!("Server failed: {:?}", e);
            }
        });

        let connector = Arc::clone(&self.connector);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let connector = Arc::clone(&connector);
                tokio::spawn(async move {
                    match connector.write(message.clone()).await {
                        Ok(response) => {
                            println!("Processed message: {:?}", response);
                        }
                        Err(e) => {
                            eprintln!("Error processing message: {:?}", e);
                        }
                    }
                });
            }
        });
        
        Ok(())
    }
}
