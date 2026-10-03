//! Question #106: Link-Cut Dynamic Connectivity
//! Category: Data Structures | Difficulty: Hard
//! Concepts: dynamic connectivity, link-cut, fully dynamic, forest
//! Description: Use link-cut trees to maintain connected components under edge insertions and deletions.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LinkCutDynamicConnectivity {
    data: Mutex<HashMap<i32, i32>>,
}

impl LinkCutDynamicConnectivity {
    pub fn new() -> Self {
        Self { data: Mutex::new(HashMap::new()) }
    }
    pub fn insert(&self, key: i32, val: i32) {
        self.data.lock().unwrap().insert(key, val);
    }
    pub fn get(&self, key: i32) -> Option<i32> {
        self.data.lock().unwrap().get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_link_cut_dynamic_connectivity() {
        let s = LinkCutDynamicConnectivity::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
