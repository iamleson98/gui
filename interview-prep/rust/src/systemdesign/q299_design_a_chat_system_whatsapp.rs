//! Question #299: Design a Chat System (WhatsApp)
//! Category: System Design | Difficulty: Hard
//! Concepts: chat, presence, delivery receipts, offline sync
//! Description: Design a real-time chat service with presence, delivery receipts, and offline sync.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAChatSystemWhatsapp {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAChatSystemWhatsapp {
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
    fn test_design_a_chat_system_whatsapp() {
        let s = DesignAChatSystemWhatsapp::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
