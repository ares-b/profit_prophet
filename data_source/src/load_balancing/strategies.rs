use async_trait::async_trait;

use crate::load_balancing::{LeastConnectionsLoadBalancingStrategy, RoundRobinLoadBalancingStrategy};

use super::LoadBalancingStrategy;

#[derive(Clone)]
pub enum LoadBalancingStrategies {
    RoundRobin,
    LeastConnections,
}

#[derive(Clone)]
pub enum LoadBalancing {
    RoundRobin(RoundRobinLoadBalancingStrategy),
    LeastConnections(LeastConnectionsLoadBalancingStrategy),
}

#[async_trait]
impl LoadBalancingStrategy for LoadBalancing {
    async fn select_listener(&mut self, listeners: &[String]) -> String {
        match self {
            LoadBalancing::RoundRobin(strategy) => strategy.select_listener(listeners).await,
            LoadBalancing::LeastConnections(strategy) => strategy.select_listener(listeners).await,
        }
    }
    fn update_state(&mut self, listener: &str, busy: bool) {
        match self {
            LoadBalancing::RoundRobin(strategy) => strategy.update_state(listener, busy),
            LoadBalancing::LeastConnections(strategy) => strategy.update_state(listener, busy),
        }
    }
}
