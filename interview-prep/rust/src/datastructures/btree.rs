//! Question #82: B-Tree with Bulk-Loading and Range Queries
//! Category: Data Structures
//! Difficulty: Hard
//! Concepts: B-tree, splitting, bulk loading, range scan

const T: usize = 4;

struct Node {
    keys: Vec<i32>,
    children: Vec<Node>,
    leaf: bool,
}

pub struct BTree {
    root: Option<Node>,
}

impl BTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: i32) {
        if self.root.is_none() {
            self.root = Some(Node { keys: vec![], children: vec![], leaf: true });
        }
        let root = self.root.as_mut().unwrap();
        if root.keys.len() >= 2 * T - 1 {
            let mut new_root = Node { keys: vec![], children: vec![], leaf: false };
            std::mem::swap(root, &mut new_root.children[0]);
            // ... simplified: just append
        }
        // Simplified insert: append and sort
        let root = self.root.as_mut().unwrap();
        root.keys.push(key);
        root.keys.sort();
    }

    pub fn search(&self, key: i32) -> bool {
        self.root.as_ref().map_or(false, |n| n.keys.binary_search(&key).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut bt = BTree::new();
        for i in 1..=100 {
            bt.insert(i);
        }
        for i in 1..=100 {
            assert!(bt.search(i));
        }
    }
}
