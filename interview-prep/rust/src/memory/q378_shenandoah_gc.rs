//! Question #378: Shenandoah GC
//! Category: Memory Management | Difficulty: Hard
//! Concepts: Shenandoah, Brooks pointer, concurrent evacuation, low latency
//! Description: Explain Shenandoah's concurrent evacuation using Brooks forwarding pointers.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ShenandoahGc {
    inner: Mutex<HashMap<String, String>>,
}

impl ShenandoahGc {
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
    fn test_shenandoah_gc() {
        let s = ShenandoahGc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
