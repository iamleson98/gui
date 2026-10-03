//! Question #103: ARC (Adaptive Replacement Cache)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: ARC, adaptive, recency, frequency
//! Description: Implement ARC, which dynamically balances recency and frequency between LRU and LFU.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ArcAdaptiveReplacementCache {
    data: Mutex<HashMap<i32, i32>>,
}

impl ArcAdaptiveReplacementCache {
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
    fn test_arc_adaptive_replacement_cache() {
        let s = ArcAdaptiveReplacementCache::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
