//! Dijkstra's Algorithm — single-source shortest path.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Graph {
    n: usize,
    adj: Vec<Vec<(usize, i64)>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self { n, adj: vec![Vec::new(); n] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.adj[from].push((to, cost));
    }

    pub fn dijkstra(&self, source: usize) -> Vec<Option<i64>> {
        let mut dist: Vec<Option<i64>> = vec![None; self.n];
        dist[source] = Some(0);
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i64, source)));
        while let Some(Reverse((d, u))) = heap.pop() {
            if let Some(du) = dist[u] {
                if d > du {
                    continue;
                }
            }
            for &(v, w) in &self.adj[u] {
                let new_dist = d + w;
                if dist[v].map_or(true, |dv| new_dist < dv) {
                    dist[v] = Some(new_dist);
                    heap.push(Reverse((new_dist, v)));
                }
            }
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 4, 5);
        g.add_edge(1, 2, 1);
        g.add_edge(1, 4, 2);
        g.add_edge(2, 3, 4);
        g.add_edge(3, 0, 7);
        g.add_edge(4, 1, 3);
        g.add_edge(4, 2, 9);
        g.add_edge(4, 3, 2);
        let dist = g.dijkstra(0);
        let expected = [Some(0), Some(8), Some(9), Some(7), Some(5)];
        for (i, d) in dist.iter().enumerate() {
            assert_eq!(*d, expected[i]);
        }
    }
}
