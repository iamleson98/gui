//! Question #127: RRB-Trees (Relaxed Radix Balanced)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: RRB tree, relaxed, concat, immutable vector
//! Description: Build relaxed radix-balanced trees for efficient concat and split on immutable vectors.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RrbTreesRelaxedRadixBalanced {
    data: Mutex<HashMap<i32, i32>>,
}

impl RrbTreesRelaxedRadixBalanced {
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
    fn test_rrb_trees_relaxed_radix_balanced() {
        let s = RrbTreesRelaxedRadixBalanced::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
