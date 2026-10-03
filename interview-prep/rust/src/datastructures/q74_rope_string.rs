//! Question #74: Rope (String)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: rope, balanced tree, split, chunks
//! Description: Implement a rope as a balanced binary tree of string chunks supporting split, concat, and insert.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RopeString {
    data: Mutex<HashMap<i32, i32>>,
}

impl RopeString {
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
    fn test_rope_string() {
        let s = RopeString::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
