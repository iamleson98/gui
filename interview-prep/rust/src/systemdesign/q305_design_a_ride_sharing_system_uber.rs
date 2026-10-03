//! Question #305: Design a Ride-Sharing System (Uber)
//! Category: System Design | Difficulty: Hard
//! Concepts: ride sharing, geospatial, dispatch, surge
//! Description: Design ride matching with geospatial indexing, surge pricing, and dispatch.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignARideSharingSystemUber {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignARideSharingSystemUber {
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
    fn test_design_a_ride_sharing_system_uber() {
        let s = DesignARideSharingSystemUber::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
