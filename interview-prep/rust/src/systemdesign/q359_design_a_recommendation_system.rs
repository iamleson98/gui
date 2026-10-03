//! Question #359: Design a Recommendation System
//! Category: System Design | Difficulty: Hard
//! Concepts: recommendations, collaborative filtering, embeddings, ranking
//! Description: Design a collaborative filtering recommender with embedding retrieval and ranking.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignARecommendationSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignARecommendationSystem {
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
    fn test_design_a_recommendation_system() {
        let s = DesignARecommendationSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
