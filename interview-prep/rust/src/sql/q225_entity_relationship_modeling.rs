//! Question #225: Entity-Relationship Modeling
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: ER modeling, entities, relationships, cardinality
//! Description: Translate an ER diagram into a normalized relational schema with keys and cardinalities.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct EntityRelationshipModeling {
    inner: Mutex<HashMap<String, String>>,
}

impl EntityRelationshipModeling {
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
    fn test_entity_relationship_modeling() {
        let s = EntityRelationshipModeling::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
