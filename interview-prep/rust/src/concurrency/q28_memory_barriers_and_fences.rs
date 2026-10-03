//! Question #28: Memory Barriers and Fences
//! Category: Concurrency | Difficulty: Hard
//! Concepts: memory fence, load-store, visibility, portability
//! Description: Place read and write fences correctly so that lock-free algorithms publish visibility and consumption in the intended order.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryBarriersAndFences {
    data: Mutex<HashMap<i32, i32>>,
}

impl MemoryBarriersAndFences {
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
    fn test_memory_barriers_and_fences() {
        let s = MemoryBarriersAndFences::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
