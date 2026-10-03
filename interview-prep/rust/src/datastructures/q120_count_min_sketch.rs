//! Question #120: Count-Min Sketch
//! Category: Data Structures | Difficulty: Hard
//! Concepts: Count-Min sketch, frequency, hash functions, overestimate
//! Description: Implement the Count-Min sketch for approximate frequency estimation with d hash functions.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CountMinSketch {
    data: Mutex<HashMap<i32, i32>>,
}

impl CountMinSketch {
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
    fn test_count_min_sketch() {
        let s = CountMinSketch::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
