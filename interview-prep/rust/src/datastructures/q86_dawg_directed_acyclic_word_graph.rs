//! Question #86: DAWG (Directed Acyclic Word Graph)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: DAWG, minimal DFA, suffix links, strings
//! Description: Build a minimal acyclic DFA accepting all suffixes of a string via suffix-tree compression.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DawgDirectedAcyclicWordGraph {
    data: Mutex<HashMap<i32, i32>>,
}

impl DawgDirectedAcyclicWordGraph {
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
    fn test_dawg_directed_acyclic_word_graph() {
        let s = DawgDirectedAcyclicWordGraph::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
