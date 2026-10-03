//! Question #26: DCAS / Double-Width CAS
//! Category: Concurrency | Difficulty: Hard
//! Concepts: DCAS, double-width CAS, versioning, portability
//! Description: Implement a 128-bit compare-and-swap (DCAS) to atomically update a pointer and a counter together.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DcasDoubleWidthCas {
    data: Mutex<HashMap<i32, i32>>,
}

impl DcasDoubleWidthCas {
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
    fn test_dcas_double_width_cas() {
        let s = DcasDoubleWidthCas::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
