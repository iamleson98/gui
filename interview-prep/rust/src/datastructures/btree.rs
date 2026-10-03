//! Question #82: B-Tree with Bulk-Loading and Range Queries
//! Category: Data Structures | Difficulty: Hard | Concepts: B-tree, splitting, bulk loading, range scan

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
        Self { root: Some(Node { keys: Vec::new(), children: Vec::new(), leaf: true }) }
    }

    pub fn insert(&mut self, key: i32) {
        if let Some(ref mut root) = self.root {
            root.keys.push(key);
            root.keys.sort();
        }
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
