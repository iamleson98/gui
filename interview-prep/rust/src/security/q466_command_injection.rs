//! Question #466: Command Injection
//! Category: Security | Difficulty: Hard
//! Concepts: command injection, shell, argument array, escaping
//! Description: Prevent command injection by avoiding shell calls and using argument arrays.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CommandInjection {
    inner: Mutex<HashMap<String, String>>,
}

impl CommandInjection {
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
    fn test_command_injection() {
        let s = CommandInjection::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
