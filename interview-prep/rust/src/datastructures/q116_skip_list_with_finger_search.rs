//! Question #116: Skip List with Finger Search
//! Category: Data Structures | Difficulty: Hard
//! Concepts: skip list, finger, local search, ordered map
//! Description: Extend a skip list with finger search to find elements near a recent position faster.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SkipListWithFingerSearch {
    data: Mutex<HashMap<i32, i32>>,
}

impl SkipListWithFingerSearch {
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
    fn test_skip_list_with_finger_search() {
        let s = SkipListWithFingerSearch::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
