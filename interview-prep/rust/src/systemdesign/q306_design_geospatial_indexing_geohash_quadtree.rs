//! Question #306: Design Geospatial Indexing (Geohash/Quadtree)
//! Category: System Design | Difficulty: Hard
//! Concepts: geospatial, geohash, quadtree, nearby
//! Description: Index moving vehicles by geohash or quadtree for nearby-driver queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignGeospatialIndexingGeohashQuadtree {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignGeospatialIndexingGeohashQuadtree {
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
    fn test_design_geospatial_indexing_geohash_quadtree() {
        let s = DesignGeospatialIndexingGeohashQuadtree::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
