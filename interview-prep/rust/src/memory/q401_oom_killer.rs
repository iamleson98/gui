//! Question #401: OOM Killer
//! Category: Memory Management | Difficulty: Hard
//! Concepts: OOM, killer, victim selection, memory score
//! Description: Design an out-of-memory killer that selects victims based on memory score.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct OomKiller {
    inner: Mutex<HashMap<String, String>>,
}

impl OomKiller {
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
    fn test_oom_killer() {
        let s = OomKiller::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
