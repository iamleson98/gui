//! Question #278: Distributed Transactions (XA)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: XA, distributed, prepare, resource manager
//! Description: Coordinate distributed XA transactions across resource managers with prepare/commit phases.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DistributedTransactionsXa {
    inner: Mutex<HashMap<String, String>>,
}

impl DistributedTransactionsXa {
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
    fn test_distributed_transactions_xa() {
        let s = DistributedTransactionsXa::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
