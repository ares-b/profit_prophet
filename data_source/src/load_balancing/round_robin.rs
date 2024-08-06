use super::{BaseLoadBalancing, LoadBalancingStrategy};
use async_trait::async_trait;
use tokio::time::{self, Duration};

#[derive(Clone)]
pub struct RoundRobinLoadBalancingStrategy {
    base: BaseLoadBalancing,
    current_index: usize,
}

impl RoundRobinLoadBalancingStrategy {
    pub fn new(listeners: &[String]) -> Self {
        let base = BaseLoadBalancing::new(listeners);
        Self { base, current_index: 0 }
    }
}

#[async_trait]
impl LoadBalancingStrategy for RoundRobinLoadBalancingStrategy {
    async fn select_listener(&mut self, listeners: &[String]) -> String {
        loop {
            let start_index = self.current_index;
            loop {
                let listener = &listeners[self.current_index];
                self.current_index = (self.current_index + 1) % listeners.len();

                if self.base.is_ready(listener) {
                    return listener.clone();
                }

                if self.current_index == start_index {
                    // All listeners are busy, break to wait
                    break;
                }
            }
            time::sleep(Duration::from_millis(100)).await;
        }
    }

    fn update_state(&mut self, listener: &str, busy: bool) {
        if busy {
            self.base.mark_busy(listener);
        } else {
            self.base.mark_ready(listener);
        }
    }
}
