//! Question #311: Design a Payment System
//! Category: System Design | Difficulty: Hard
//! Concepts: payments, ledger, idempotency, reconciliation
//! Description: Design a payment processing system with idempotency, ledgering, and reconciliation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAPaymentSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAPaymentSystem {
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
    fn test_design_a_payment_system() {
        let s = DesignAPaymentSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
