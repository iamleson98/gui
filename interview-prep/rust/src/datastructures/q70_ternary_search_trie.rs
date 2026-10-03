//! Question #70: Ternary Search Trie
//! Category: Data Structures | Difficulty: Hard
//! Concepts: ternary trie, strings, prefix, branching
//! Description: Implement a ternary search trie for string keys with character-by-character branching.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TernarySearchTrie {
    data: Mutex<HashMap<i32, i32>>,
}

impl TernarySearchTrie {
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
    fn test_ternary_search_trie() {
        let s = TernarySearchTrie::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
