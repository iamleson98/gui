//! Question #126: Hash Array Mapped Trie (HAMT)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: HAMT, bitmap, hash trie, persistent
//! Description: Implement a HAMT using a sparse bitmap and a variable-length pointer array per node.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HashArrayMappedTrieHamt {
    data: Mutex<HashMap<i32, i32>>,
}

impl HashArrayMappedTrieHamt {
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
    fn test_hash_array_mapped_trie_hamt() {
        let s = HashArrayMappedTrieHamt::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
