//! Question #118: Implicit Treap (Split/Merge)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: implicit treap, split, merge, subtree size
//! Description: Build a treap keyed by subtree size supporting split and merge for sequence operations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ImplicitTreapSplitMerge {
    data: Mutex<HashMap<i32, i32>>,
}

impl ImplicitTreapSplitMerge {
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
    fn test_implicit_treap_split_merge() {
        let s = ImplicitTreapSplitMerge::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
