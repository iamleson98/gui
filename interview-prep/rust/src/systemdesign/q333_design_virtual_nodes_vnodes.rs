//! Question #333: Design Virtual Nodes (Vnodes)
//! Category: System Design | Difficulty: Hard
//! Concepts: vnodes, load balancing, hotspots, ring
//! Description: Add virtual nodes to consistent hashing to smooth load and reduce hotspots.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignVirtualNodesVnodes {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignVirtualNodesVnodes {
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
    fn test_design_virtual_nodes_vnodes() {
        let s = DesignVirtualNodesVnodes::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
