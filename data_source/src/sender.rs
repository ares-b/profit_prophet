use tokio::sync::mpsc;
use tokio::task;
use std::collections::HashMap;
use crate::load_balancing::{LoadBalancing, LoadBalancingStrategies, LeastConnectionsLoadBalancingStrategy, RoundRobinLoadBalancingStrategy, LoadBalancingStrategy};
use connector::Message;
use connector::tcp::client::Client;

pub struct DataSender {
    addresses: Vec<String>,
    load_balancing: LoadBalancing,
    client_senders: HashMap<String, mpsc::Sender<Message>>,
    receiver: mpsc::Receiver<Message>,
}

impl DataSender {
    pub fn new(
        addresses: Vec<String>,
        load_balancing_strategy: Option<LoadBalancingStrategies>,
        receiver: mpsc::Receiver<Message>
    ) -> Self {
        let load_balancing = match load_balancing_strategy {
            None => LoadBalancing::LeastConnections(LeastConnectionsLoadBalancingStrategy::new(&addresses)),
            Some(strategy) => match strategy {
                LoadBalancingStrategies::RoundRobin => LoadBalancing::RoundRobin(RoundRobinLoadBalancingStrategy::new(&addresses)),
                LoadBalancingStrategies::LeastConnections => LoadBalancing::LeastConnections(LeastConnectionsLoadBalancingStrategy::new(&addresses)),
            }
        };

        let mut client_senders = HashMap::new();
        for addr in addresses.iter() {
            let (tx, rx) = mpsc::channel(100);
            let client = Client::new(addr, rx);
            task::spawn(client.start());
            client_senders.insert(addr.clone(), tx);
        }

        Self {
            addresses,
            load_balancing,
            client_senders,
            receiver
        }
    }

    pub async fn start(&mut self) {
        while let Some(data) = self.receiver.recv().await {
            let listener = self.load_balancing.select_listener(&self.addresses).await;
            if let Some(sender) = self.client_senders.get(&listener) {
                if let Err(e) = sender.send(data).await {
                    eprintln!("Failed to send data to {}: {:?}", listener, e);
                }
            } else {
                eprintln!("No sender found for listener {:?}", listener);
            }
        }
    }
}
