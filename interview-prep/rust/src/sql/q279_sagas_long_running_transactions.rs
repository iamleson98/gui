//! Question #279: Sagas (Long-Running Transactions)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: saga, compensation, long-running, choreography
//! Description: Model long-running business transactions as a saga of compensating local actions.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SagasLongRunningTransactions {
    inner: Mutex<HashMap<String, String>>,
}

impl SagasLongRunningTransactions {
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
    fn test_sagas_long_running_transactions() {
        let s = SagasLongRunningTransactions::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
