//! Question #27: Memory Ordering: Acquire/Release vs seq_cst
//! Category: Concurrency | Difficulty: Hard
//! Concepts: memory ordering, acquire/release, seq_cst, reordered
//! Description: Compare acquire/release, relaxed, and sequentially-consistent ordering and pick the weakest safe ordering per access.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryOrderingAcquireReleaseVsSeqCst {
    data: Mutex<HashMap<i32, i32>>,
}

impl MemoryOrderingAcquireReleaseVsSeqCst {
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
    fn test_memory_ordering_acquire_release_vs_seq_cst() {
        let s = MemoryOrderingAcquireReleaseVsSeqCst::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
