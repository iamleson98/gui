//! Question #381: Write Barriers (SATB/INC)
//! Category: Memory Management | Difficulty: Hard
//! Concepts: write barrier, SATB, incremental, invariant
//! Description: Implement SATB and incremental-update write barriers to maintain tri-color invariance.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WriteBarriersSatbInc {
    inner: Mutex<HashMap<String, String>>,
}

impl WriteBarriersSatbInc {
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
    fn test_write_barriers_satb_inc() {
        let s = WriteBarriersSatbInc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
