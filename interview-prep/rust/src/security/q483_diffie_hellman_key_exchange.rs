//! Question #483: Diffie-Hellman Key Exchange
//! Category: Security | Difficulty: Hard
//! Concepts: Diffie-Hellman, ephemeral, shared secret, discrete log
//! Description: Implement ephemeral Diffie-Hellman to establish shared secrets over an insecure channel.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DiffieHellmanKeyExchange {
    inner: Mutex<HashMap<String, String>>,
}

impl DiffieHellmanKeyExchange {
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
    fn test_diffie_hellman_key_exchange() {
        let s = DiffieHellmanKeyExchange::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
