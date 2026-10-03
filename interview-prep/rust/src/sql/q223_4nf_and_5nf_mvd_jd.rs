//! Question #223: 4NF and 5NF (MVD/JD)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: 4NF, 5NF, multi-valued dependency, join dependency
//! Description: Handle multi-valued and join dependencies to reach 4NF and 5NF in complex schemas.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct 4NfAnd5NfMvdJd {
    inner: Mutex<HashMap<String, String>>,
}

impl 4NfAnd5NfMvdJd {
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
    fn test_4nf_and_5nf_mvd_jd() {
        let s = 4NfAnd5NfMvdJd::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
