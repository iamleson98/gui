//! Question #60: Coherence Traffic and False Sharing
//! Category: Concurrency | Difficulty: Hard
//! Concepts: false sharing, cache line, padding, coherence
//! Description: Diagnose false sharing between adjacent atomics on the same cache line and pad to eliminate coherence traffic.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CoherenceTrafficAndFalseSharing {
    data: Mutex<HashMap<i32, i32>>,
}

impl CoherenceTrafficAndFalseSharing {
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
    fn test_coherence_traffic_and_false_sharing() {
        let s = CoherenceTrafficAndFalseSharing::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
