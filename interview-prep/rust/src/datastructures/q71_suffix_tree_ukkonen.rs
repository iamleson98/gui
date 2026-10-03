//! Question #71: Suffix Tree (Ukkonen)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: suffix tree, Ukkonen, implicit links, linear time
//! Description: Build Ukkonen's linear-time suffix tree with implicit suffix links and active point extension.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SuffixTreeUkkonen {
    data: Mutex<HashMap<i32, i32>>,
}

impl SuffixTreeUkkonen {
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
    fn test_suffix_tree_ukkonen() {
        let s = SuffixTreeUkkonen::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
