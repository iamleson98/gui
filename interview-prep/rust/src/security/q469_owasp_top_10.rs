//! Question #469: OWASP Top 10
//! Category: Security | Difficulty: Hard
//! Concepts: OWASP, Top 10, risk, mitigation
//! Description: Map a system's defenses to the OWASP Top 10 risk categories.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct OwaspTop10 {
    inner: Mutex<HashMap<String, String>>,
}

impl OwaspTop10 {
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
    fn test_owasp_top_10() {
        let s = OwaspTop10::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
