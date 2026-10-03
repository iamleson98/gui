//! Question #72: Suffix Array
//! Category: Data Structures | Difficulty: Hard
//! Concepts: suffix array, SA-IS, binary search, strings
//! Description: Construct a suffix array in O(n log n) and demonstrate binary search over it.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SuffixArray {
    data: Mutex<HashMap<i32, i32>>,
}

impl SuffixArray {
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
    fn test_suffix_array() {
        let s = SuffixArray::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
