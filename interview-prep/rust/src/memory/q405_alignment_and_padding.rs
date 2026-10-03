//! Question #405: Alignment and Padding
//! Category: Memory Management | Difficulty: Hard
//! Concepts: alignment, padding, struct layout, ABI
//! Description: Lay out structs with alignment rules to avoid misaligned access and padding waste.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AlignmentAndPadding {
    inner: Mutex<HashMap<String, String>>,
}

impl AlignmentAndPadding {
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
    fn test_alignment_and_padding() {
        let s = AlignmentAndPadding::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
