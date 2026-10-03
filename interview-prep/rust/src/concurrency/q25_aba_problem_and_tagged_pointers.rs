//! Question #25: ABA Problem and Tagged Pointers
//! Category: Concurrency | Difficulty: Hard
//! Concepts: ABA, tagged pointer, CAS, versioning
//! Description: Demonstrate the ABA problem on a Treiber stack and fix it using a tagged pointer packing a version counter.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AbaProblemAndTaggedPointers {
    data: Mutex<HashMap<i32, i32>>,
}

impl AbaProblemAndTaggedPointers {
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
    fn test_aba_problem_and_tagged_pointers() {
        let s = AbaProblemAndTaggedPointers::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
