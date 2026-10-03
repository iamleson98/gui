//! Question #85: Compressed Trie (Patricia Trie)
//! Category: Data Structures
//! Difficulty: Hard
//! Concepts: Patricia trie, path compression, sparse keys

use std::collections::HashMap;

struct Node {
    children: HashMap<u8, Node>,
    is_end: bool,
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Node { children: HashMap::new(), is_end: false } }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            node = node.children.entry(b).or_insert(Node { children: HashMap::new(), is_end: false });
        }
        node.is_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for b in word.bytes() {
            match node.children.get(&b) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for b in prefix.bytes() {
            match node.children.get(&b) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut t = Trie::new();
        t.insert("apple");
        assert!(t.search("apple"));
        assert!(!t.search("app"));
        assert!(t.starts_with("app"));
    }
}
