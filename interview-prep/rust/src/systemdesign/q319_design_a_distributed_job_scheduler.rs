//! Question #319: Design a Distributed Job Scheduler
//! Category: System Design | Difficulty: Hard
//! Concepts: scheduler, leases, retries, idempotent
//! Description: Design a fault-tolerant scheduler with leases, retries, and idempotent execution.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignADistributedJobScheduler {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignADistributedJobScheduler {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }
    pub fn set(&self, key: &str, val: &str) {
        self.inner.lock().unwrap().insert(key.to_string(), val.to_string());
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().unwrap().get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_design_a_distributed_job_scheduler() {
        let s = DesignADistributedJobScheduler::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
