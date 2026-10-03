//! Question #345: Design Disaster Recovery (RPO/RTO)
//! Category: System Design | Difficulty: Hard
//! Concepts: DR, RPO, RTO, failover
//! Description: Quantify RPO/RTO and design backup, replication, and failover to meet them.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignDisasterRecoveryRpoRto {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignDisasterRecoveryRpoRto {
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
    fn test_design_disaster_recovery_rpo_rto() {
        let s = DesignDisasterRecoveryRpoRto::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
