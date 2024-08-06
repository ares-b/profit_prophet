use super::{BaseLoadBalancing, LoadBalancingStrategy};
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::time::{self, Duration};

#[derive(Clone)]
pub struct LeastConnectionsLoadBalancingStrategy {
    base: BaseLoadBalancing,
    connections: HashMap<String, usize>,
}

impl LeastConnectionsLoadBalancingStrategy {
    pub fn new(listeners: &[String]) -> Self {
        let base = BaseLoadBalancing::new(listeners);
        let connections = listeners.iter().cloned().map(|l| (l, 0)).collect();
        Self { base, connections }
    }

    pub fn increment_connection(&mut self, listener: &str) {
        if let Some(count) = self.connections.get_mut(listener) {
            *count += 1;
        }
    }

    pub fn decrement_connection(&mut self, listener: &str) {
        if let Some(count) = self.connections.get_mut(listener) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }
}

#[async_trait]
impl LoadBalancingStrategy for LeastConnectionsLoadBalancingStrategy {
    async fn select_listener(&mut self, listeners: &[String]) -> String {
        loop {
            if let Some(listener) = listeners
                .iter()
                .filter(|&listener| self.base.is_ready(listener))
                .min_by_key(|&listener| self.connections.get(listener).unwrap_or(&0))
                .cloned()
            {
                self.increment_connection(&listener);
                return listener;
            }
            time::sleep(Duration::from_millis(100)).await;
        }
    }

    fn update_state(&mut self, listener: &str, busy: bool) {
        if busy {
            self.base.mark_busy(listener);
        } else {
            self.base.mark_ready(listener);
            self.decrement_connection(listener);
        }
    }
}
