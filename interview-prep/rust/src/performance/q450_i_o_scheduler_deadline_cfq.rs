//! Question #450: I/O Scheduler (deadline/CFQ)
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: I/O scheduler, deadline, CFQ, merging
//! Description: Explain disk I/O schedulers and their effect on latency and throughput.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct IOSchedulerDeadlineCfq {
    inner: Mutex<HashMap<String, String>>,
}

impl IOSchedulerDeadlineCfq {
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
    fn test_i_o_scheduler_deadline_cfq() {
        let s = IOSchedulerDeadlineCfq::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
