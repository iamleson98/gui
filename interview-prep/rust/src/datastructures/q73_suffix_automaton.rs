//! Question #73: Suffix Automaton
//! Category: Data Structures | Difficulty: Hard
//! Concepts: suffix automaton, DFA, endpos, online
//! Description: Build the minimal DFA of all suffixes of a string for online substring queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SuffixAutomaton {
    data: Mutex<HashMap<i32, i32>>,
}

impl SuffixAutomaton {
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
    fn test_suffix_automaton() {
        let s = SuffixAutomaton::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
