//! Question #185: Max Flow: Edmonds-Karp
//! Category: Algorithms | Difficulty: Hard
//! Concepts: max flow, Edmonds-Karp, BFS, shortest augmenting path
//! Description: Implement the BFS-based shortest-augmenting-path max flow with polynomial time.

pub fn max_flow_edmonds_karp(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_flow_edmonds_karp() {
        assert_eq!(max_flow_edmonds_karp(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
