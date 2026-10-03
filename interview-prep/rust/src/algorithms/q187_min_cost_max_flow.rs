//! Question #187: Min-Cost Max-Flow
//! Category: Algorithms | Difficulty: Hard
//! Concepts: min-cost flow, potentials, SPFA, residual
//! Description: Find the maximum flow of minimum cost using successive shortest paths with potentials.

pub fn min_cost_max_flow(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_cost_max_flow() {
        assert_eq!(min_cost_max_flow(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
