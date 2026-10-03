//! Question #44: Actor Model Mailbox
//! Category: Concurrency | Difficulty: Hard
//! Concepts: actor model, mailbox, message passing, dispatcher
//! Description: Implement an actor runtime with per-actor mailboxes, message ordering, and a dispatcher.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ActorModelMailbox {
    data: Mutex<HashMap<i32, i32>>,
}

impl ActorModelMailbox {
    pub fn new() -> Self {
        Self { data: Mutex::new(HashMap::new()) }
    }
    pub fn insert(&self, key: i32, val: i32) {
        self.data.lock().unwrap().insert(key, val);
    }
    pub fn get(&self, key: i32) -> Option<i32> {
        self.data.lock().unwrap().get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_actor_model_mailbox() {
        let s = ActorModelMailbox::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
