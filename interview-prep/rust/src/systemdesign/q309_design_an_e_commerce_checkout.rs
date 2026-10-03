//! Question #309: Design an E-Commerce Checkout
//! Category: System Design | Difficulty: Hard
//! Concepts: checkout, cart, pricing, payment
//! Description: Design a checkout pipeline with cart, pricing, inventory, and payment orchestration.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAnECommerceCheckout {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAnECommerceCheckout {
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
    fn test_design_an_e_commerce_checkout() {
        let s = DesignAnECommerceCheckout::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
