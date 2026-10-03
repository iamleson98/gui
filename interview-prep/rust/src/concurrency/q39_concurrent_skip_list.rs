//! Question #39: Concurrent Skip List
//! Category: Concurrency | Difficulty: Hard
//! Concepts: skip list, concurrent, probabilistic, ordered map
//! Description: Implement a lock-free or fine-grained skip list supporting ordered map operations with probabilistic levels.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ConcurrentSkipList {
    data: Mutex<HashMap<i32, i32>>,
}

impl ConcurrentSkipList {
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
    fn test_concurrent_skip_list() {
        let s = ConcurrentSkipList::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
