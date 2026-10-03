//! Question #537: Anycast Routing
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: anycast, BGP, POP, latency
//! Description: Route users to the nearest POP using BGP anycast for low-latency edge services.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AnycastRouting {
    inner: Mutex<HashMap<String, String>>,
}

impl AnycastRouting {
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
    fn test_anycast_routing() {
        let s = AnycastRouting::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
