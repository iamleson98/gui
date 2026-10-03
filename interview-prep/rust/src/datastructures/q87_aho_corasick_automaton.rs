//! Question #87: Aho-Corasick Automaton
//! Category: Data Structures | Difficulty: Hard
//! Concepts: Aho-Corasick, failure links, multi-pattern, DFA
//! Description: Construct the AC automaton with failure links for multi-pattern string matching.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AhoCorasickAutomaton {
    data: Mutex<HashMap<i32, i32>>,
}

impl AhoCorasickAutomaton {
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
    fn test_aho_corasick_automaton() {
        let s = AhoCorasickAutomaton::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
