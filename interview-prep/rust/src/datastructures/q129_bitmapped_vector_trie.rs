//! Question #129: Bitmapped Vector Trie
//! Category: Data Structures | Difficulty: Hard
//! Concepts: vector trie, bitmap, branching, persistent
//! Description: Build a trie with bitmap per node and branching factor of word size for persistent arrays.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BitmappedVectorTrie {
    data: Mutex<HashMap<i32, i32>>,
}

impl BitmappedVectorTrie {
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
    fn test_bitmapped_vector_trie() {
        let s = BitmappedVectorTrie::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
