//! Question #113: VList
//! Category: Data Structures | Difficulty: Hard
//! Concepts: VList, linked blocks, persistent, indexing
//! Description: Implement the VList structure providing O(1) cons and O(log n) indexing using linked blocks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Vlist {
    data: Mutex<HashMap<i32, i32>>,
}

impl Vlist {
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
    fn test_vlist() {
        let s = Vlist::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
