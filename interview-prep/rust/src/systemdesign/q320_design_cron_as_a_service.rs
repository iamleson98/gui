//! Question #320: Design Cron-as-a-Service
//! Category: System Design | Difficulty: Hard
//! Concepts: cron, multi-tenant, scheduling, workers
//! Description: Design a multi-tenant cron service distributing timed jobs across workers.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignCronAsAService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignCronAsAService {
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
    fn test_design_cron_as_a_service() {
        let s = DesignCronAsAService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
