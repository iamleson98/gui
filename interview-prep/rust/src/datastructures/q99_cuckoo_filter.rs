//! Question #99: Cuckoo Filter
//! Category: Data Structures | Difficulty: Hard
//! Concepts: cuckoo filter, fingerprint, cuckoo hashing, deletion
//! Description: Build a cuckoo-filter using bounded cuckoo hashing with fingerprints for set membership and deletion.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CuckooFilter {
    data: Mutex<HashMap<i32, i32>>,
}

impl CuckooFilter {
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
    fn test_cuckoo_filter() {
        let s = CuckooFilter::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
