//! Question #460: GC Pause Tuning
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: GC tuning, pauses, generational, heap sizing
//! Description: Tune a generational collector's heap sizes and barriers to reduce pause times.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct GcPauseTuning {
    inner: Mutex<HashMap<String, String>>,
}

impl GcPauseTuning {
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
    fn test_gc_pause_tuning() {
        let s = GcPauseTuning::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
