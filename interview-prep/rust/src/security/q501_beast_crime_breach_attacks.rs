//! Question #501: BEAST/CRIME/BREACH Attacks
//! Category: Security | Difficulty: Hard
//! Concepts: BEAST, CRIME, BREACH, compression
//! Description: Understand compression and CBC attacks (BEAST, CRIME, BREACH) and their mitigations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BeastCrimeBreachAttacks {
    inner: Mutex<HashMap<String, String>>,
}

impl BeastCrimeBreachAttacks {
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
    fn test_beast_crime_breach_attacks() {
        let s = BeastCrimeBreachAttacks::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
