//! Question #231: Surrogate vs Natural Keys
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: surrogate key, natural key, stability, joins
//! Description: Choose between surrogate and natural keys, weighing stability, size, and join performance.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SurrogateVsNaturalKeys {
    inner: Mutex<HashMap<String, String>>,
}

impl SurrogateVsNaturalKeys {
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
    fn test_surrogate_vs_natural_keys() {
        let s = SurrogateVsNaturalKeys::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
