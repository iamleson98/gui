//! Question #312: Design Idempotent Payment Processing
//! Category: System Design | Difficulty: Hard
//! Concepts: idempotency, payment, dedup, ledger
//! Description: Ensure payment APIs are idempotent using idempotency keys and a dedup store.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignIdempotentPaymentProcessing {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignIdempotentPaymentProcessing {
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
    fn test_design_idempotent_payment_processing() {
        let s = DesignIdempotentPaymentProcessing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
