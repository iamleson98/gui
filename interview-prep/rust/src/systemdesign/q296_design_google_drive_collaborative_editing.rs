//! Question #296: Design Google Drive Collaborative Editing
//! Category: System Design | Difficulty: Hard
//! Concepts: collaborative editing, OT, CRDT, conflict
//! Description: Design real-time collaborative editing using operational transformation or CRDTs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignGoogleDriveCollaborativeEditing {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignGoogleDriveCollaborativeEditing {
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
    fn test_design_google_drive_collaborative_editing() {
        let s = DesignGoogleDriveCollaborativeEditing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
