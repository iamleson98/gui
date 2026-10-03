//! Question #43: Fork-Join Pool
//! Category: Concurrency | Difficulty: Hard
//! Concepts: fork-join, work stealing, recursive, divide and conquer
//! Description: Design a fork-join executor with work-stealing deques, barrier joins, and recursive task splitting.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ForkJoinPool {
    data: Mutex<HashMap<i32, i32>>,
}

impl ForkJoinPool {
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
    fn test_fork_join_pool() {
        let s = ForkJoinPool::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
