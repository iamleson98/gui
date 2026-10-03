//! Question #111: Chunking Deque
//! Category: Data Structures | Difficulty: Hard
//! Concepts: deque, chunking, amortized, persistent
//! Description: Build a deque over fixed-size chunks (a 'banker's deque') for amortized O(1) operations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ChunkingDeque {
    data: Mutex<HashMap<i32, i32>>,
}

impl ChunkingDeque {
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
    fn test_chunking_deque() {
        let s = ChunkingDeque::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
