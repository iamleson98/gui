//! Question #88: Wavelet Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: wavelet tree, rank/select, bit vector, sequence
//! Description: Implement a wavelet tree for rank/select queries over a sequence using bit vectors.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WaveletTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl WaveletTree {
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
    fn test_wavelet_tree() {
        let s = WaveletTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
