use tokio::time::{Duration, interval_at, Instant};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use log::{debug, error};
use std::fmt::Debug;
use std::sync::Arc;
use crate::load_balancing::LoadBalancingStrategies;
use crate::sender::DataSender;
use connector::{DataConnector, Message, DataConnectorError};

#[derive(Debug, thiserror::Error)]
pub enum DataSourceError {
    #[error("Connector error: {0}")]
    ConnectorError(#[from] DataConnectorError),
    #[error("TCP error: {0}")]
    TcpError(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] serde_json::Error),
}

pub struct DataSource {
    connector: Arc<dyn DataConnector>,
    query: Message,
    interval: Option<Duration>,
    sinks: Option<Vec<String>>,
    load_balancing_strategy: Option<LoadBalancingStrategies>,
    transform: Option<Arc<dyn Fn(Message) -> Message + Send + Sync>>,
    cancellation_token: CancellationToken,
}

impl DataSource {
    pub fn new(
        connector: Arc<dyn DataConnector>,
        query: Message,
        interval: Option<Duration>,
        sinks: Option<Vec<String>>,
        load_balancing_strategy: Option<LoadBalancingStrategies>,
        transform: Option<Arc<dyn Fn(Message) -> Message + Send + Sync>>,
    ) -> Self {
        let data_source = DataSource {
            connector,
            query,
            interval,
            sinks,
            load_balancing_strategy,
            transform,
            cancellation_token: CancellationToken::new(),
        };

        debug!(
            "Initialized DataSource with Query: {:?}, Interval: {:?}, Sinks: {:?}",
            data_source.query,
            data_source.interval,
            data_source.sinks,
        );

        data_source
    }

    pub fn stop(&self) {
        self.cancellation_token.cancel()
    }

    pub async fn start(&self) -> Result<(), DataSourceError> {
        let tx = if let Some(sinks) = &self.sinks {
            let (tx, rx) = mpsc::channel::<Message>(100);
            let sinks = sinks.clone();

            let mut data_sender = DataSender::new(sinks, self.load_balancing_strategy.clone(), rx);
            tokio::spawn(async move {
                data_sender.start().await;
            });

            Some(tx)
        } else {
            None
        };

        if let Some(interval) = self.interval {
            let mut interval = interval_at(Instant::now() + interval, interval);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        self.spawn_data_source_task(tx.clone()).await?;
                    }
                    _ = self.cancellation_token.cancelled() => {
                        break;
                    }
                }
            }
        } else {
            self.spawn_data_source_task(tx).await?;
        }
        Ok(())
    }

    async fn spawn_data_source_task(&self, sender: Option<mpsc::Sender<Message>>) -> Result<(), DataSourceError> {
        let connector = Arc::clone(&self.connector);
        let query = self.query.clone();
        let transform = self.transform.clone();
        
        let handle = tokio::spawn(async move {
            match connector.read(query).await {
                Ok(mut response) => {
                    if let Some(transform_fn) = transform {
                        response = transform_fn(response);
                    }
                    if let Some(sender) = sender {
                        if let Err(e) = sender.send(response).await {
                            error!("Error sending to DataSender: {:?}", e);
                        }
                    } else {
                        debug!("Sent response: {:?}", response);
                    }
                }
                Err(e) => error!("Error querying endpoint: {:?}", e),
            }
        });

        handle.await.map_err(|e| {
            error!("Task join error: {:?}", e);
            DataSourceError::TcpError(std::io::Error::new(std::io::ErrorKind::Other, e))
        })
    }
}
