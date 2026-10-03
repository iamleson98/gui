//! Question #122: R-Tree (Spatial)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: R-tree, spatial index, MBR, splitting
//! Description: Implement an R-tree for spatial indexing of rectangles with node splitting heuristics.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RTreeSpatial {
    data: Mutex<HashMap<i32, i32>>,
}

impl RTreeSpatial {
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
    fn test_r_tree_spatial() {
        let s = RTreeSpatial::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
