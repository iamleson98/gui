//! Question #546: MQTT for IoT
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: MQTT, QoS, topics, retained
//! Description: Use MQTT topics, QoS levels, and retained messages for constrained IoT devices.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MqttForIot {
    inner: Mutex<HashMap<String, String>>,
}

impl MqttForIot {
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
    fn test_mqtt_for_iot() {
        let s = MqttForIot::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
