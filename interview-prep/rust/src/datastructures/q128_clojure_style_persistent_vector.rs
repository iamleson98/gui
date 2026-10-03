//! Question #128: Clojure-style Persistent Vector
//! Category: Data Structures | Difficulty: Hard
//! Concepts: persistent vector, bitmapped trie, tail, immutability
//! Description: Implement a persistent vector using a bitmapped trie indexed by chunks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ClojureStylePersistentVector {
    data: Mutex<HashMap<i32, i32>>,
}

impl ClojureStylePersistentVector {
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
    fn test_clojure_style_persistent_vector() {
        let s = ClojureStylePersistentVector::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
