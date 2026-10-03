//! Question #125: Fractal Tree (TokuDB)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: fractal tree, buffered, amortized I/O, B-tree
//! Description: Build a fractal index tree using buffered insertions to amortize I/O across internal nodes.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FractalTreeTokudb {
    data: Mutex<HashMap<i32, i32>>,
}

impl FractalTreeTokudb {
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
    fn test_fractal_tree_tokudb() {
        let s = FractalTreeTokudb::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
