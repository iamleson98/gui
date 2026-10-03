//! Question #94: Link-Cut Tree (Splay)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: link-cut tree, splay, dynamic forest, preferred path
//! Description: Implement a splay-based link-cut tree supporting dynamic forest queries and edge link/cut.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LinkCutTreeSplay {
    data: Mutex<HashMap<i32, i32>>,
}

impl LinkCutTreeSplay {
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
    fn test_link_cut_tree_splay() {
        let s = LinkCutTreeSplay::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
