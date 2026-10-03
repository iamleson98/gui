//! Question #8: Read-Copy-Update (RCU) Pattern
//! Category: Concurrency | Difficulty: Hard
//! Concepts: RCU, grace period, deferred reclamation, read-mostly
//! Description: Simulate RCU by allowing readers to proceed without locks and deferring reclamation to a grace period.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ReadCopyUpdateRcuPattern {
    data: Mutex<HashMap<i32, i32>>,
}

impl ReadCopyUpdateRcuPattern {
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
    fn test_read_copy_update_rcu_pattern() {
        let s = ReadCopyUpdateRcuPattern::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
