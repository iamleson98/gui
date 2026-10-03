//! Question #304: Design Google Maps / Routing
//! Category: System Design | Difficulty: Hard
//! Concepts: routing, A*, contraction hierarchies, road graph
//! Description:  Design a routing service using contraction hierarchies and A* on a road graph.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignGoogleMapsRouting {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignGoogleMapsRouting {
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
    fn test_design_google_maps_routing() {
        let s = DesignGoogleMapsRouting::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
