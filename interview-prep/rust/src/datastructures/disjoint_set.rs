//! Disjoint Set (Union-Find) with path compression + union by rank.
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
        true
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut ds = DisjointSet::new(10);
        ds.union(0, 1);
        ds.union(2, 3);
        ds.union(1, 3);
        assert!(ds.connected(0, 2));
        assert!(!ds.connected(0, 4));
    }

    #[test]
    fn test_all_union() {
        let mut ds = DisjointSet::new(100);
        for i in 0..99 {
            ds.union(i, i + 1);
        }
        let root = ds.find(0);
        for i in 1..100 {
            assert_eq!(ds.find(i), root);
        }
    }
}
