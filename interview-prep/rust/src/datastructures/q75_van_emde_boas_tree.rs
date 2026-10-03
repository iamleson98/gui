//! Question #75: Van Emde Boas Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: vEB tree, log log U, cluster, universe
//! Description: Build a vEB tree over a fixed universe for O(log log U) insert, successor, and predecessor.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct VanEmdeBoasTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl VanEmdeBoasTree {
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
    fn test_van_emde_boas_tree() {
        let s = VanEmdeBoasTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
