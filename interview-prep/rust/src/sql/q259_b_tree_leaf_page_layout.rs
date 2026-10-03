//! Question #259: B+ Tree Leaf Page Layout
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: B+ tree, leaf page, pointers, links
//! Description: Describe the leaf page layout of a B+ tree including key order, row pointers, and links.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BTreeLeafPageLayout {
    inner: Mutex<HashMap<String, String>>,
}

impl BTreeLeafPageLayout {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }
    pub fn set(&self, key: &str, val: &str) {
        self.inner.lock().unwrap().insert(key.to_string(), val.to_string());
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().unwrap().get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_b_tree_leaf_page_layout() {
        let s = BTreeLeafPageLayout::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
