//! Question #183: Max Flow: Dinic's Algorithm
//! Category: Algorithms
//! Difficulty: Hard
//! Concepts: Dinic, level graph, blocking flow, current arc

use std::collections::VecDeque;

struct Edge {
    to: usize,
    cap: i64,
    rev: usize,
}

pub struct MaxFlow {
    n: usize,
    graph: Vec<Vec<Edge>>,
}

impl MaxFlow {
    pub fn new(n: usize) -> Self {
        Self { n, graph: (0..n).map(|_| Vec::new()).collect() }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let rev_from = self.graph[from].len();
        let rev_to = self.graph[to].len();
        self.graph[from].push(Edge { to, cap, rev: rev_to });
        self.graph[to].push(Edge { to: from, cap: 0, rev: rev_from });
    }

    fn bfs(&self, s: usize, t: usize, level: &mut [i32]) -> bool {
        for l in level.iter_mut() { *l = -1; }
        level[s] = 0;
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for e in &self.graph[u] {
                if e.cap > 0 && level[e.to] < 0 {
                    level[e.to] = level[u] + 1;
                    q.push_back(e.to);
                }
            }
        }
        level[t] >= 0
    }

    fn dfs(&mut self, u: usize, t: usize, f: i64, level: &[i32], iter: &mut [usize]) -> i64 {
        if u == t { return f; }
        while iter[u] < self.graph[u].len() {
            let (to, cap, rev) = {
                let e = &self.graph[u][iter[u]];
                (e.to, e.cap, e.rev)
            };
            if cap > 0 && level[to] == level[u] + 1 {
                let d = self.dfs(to, t, f.min(cap), level, iter);
                if d > 0 {
                    self.graph[u][iter[u]].cap -= d;
                    self.graph[to][rev].cap += d;
                    return d;
                }
            }
            iter[u] += 1;
        }
        0
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0i64;
        let mut level = vec![0i32; self.n];
        let mut iter = vec![0usize; self.n];
        while self.bfs(s, t, &mut level) {
            for i in iter.iter_mut() { *i = 0; }
            loop {
                let f = self.dfs(s, t, i64::MAX >> 2, &level, &mut iter);
                if f == 0 { break; }
                flow += f;
            }
        }
        flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut mf = MaxFlow::new(4);
        mf.add_edge(0, 1, 3);
        mf.add_edge(0, 2, 2);
        mf.add_edge(1, 2, 1);
        mf.add_edge(1, 3, 2);
        mf.add_edge(2, 3, 3);
        assert_eq!(mf.max_flow(0, 3), 5);
    }
}
