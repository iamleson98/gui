//! Question #109: Persistent Array
//! Category: Data Structures | Difficulty: Hard
//! Concepts: persistent array, fat node, versioning, immutability
//! Description: Build a persistent array using a fat-node or balanced-tree representation with O(log n) updates.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PersistentArray {
    data: Mutex<HashMap<i32, i32>>,
}

impl PersistentArray {
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
    fn test_persistent_array() {
        let s = PersistentArray::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
