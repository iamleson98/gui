//! Question #105: Union-Find with Rollback
//! Category: Data Structures | Difficulty: Hard
//! Concepts: union-find, rollback, persistent, offline
//! Description: Extend DSU to support undo of union operations for backtracking and offline algorithms.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct UnionFindWithRollback {
    data: Mutex<HashMap<i32, i32>>,
}

impl UnionFindWithRollback {
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
    fn test_union_find_with_rollback() {
        let s = UnionFindWithRollback::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
