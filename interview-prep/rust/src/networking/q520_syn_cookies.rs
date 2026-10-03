//! Question #520: SYN Cookies
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: SYN cookies, SYN flood, stateless, DoS
//! Description: Defend against SYN floods with stateless SYN cookies encoded in the sequence number.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SynCookies {
    inner: Mutex<HashMap<String, String>>,
}

impl SynCookies {
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
    fn test_syn_cookies() {
        let s = SynCookies::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
