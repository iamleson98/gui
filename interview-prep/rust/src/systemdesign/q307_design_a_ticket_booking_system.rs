//! Question #307: Design a Ticket Booking System
//! Category: System Design | Difficulty: Hard
//! Concepts: booking, seat hold, idempotency, overbooking
//! Description: Design a seat-booking service with hold locks, idempotency, and overbooking prevention.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignATicketBookingSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignATicketBookingSystem {
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
    fn test_design_a_ticket_booking_system() {
        let s = DesignATicketBookingSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
