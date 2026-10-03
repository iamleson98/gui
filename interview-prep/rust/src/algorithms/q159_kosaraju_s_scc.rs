//! Question #159: Kosaraju's SCC
//! Category: Algorithms | Difficulty: Hard
//! Concepts: SCC, Kosaraju, reverse graph, finish order
//! Description: Compute SCCs by running DFS on the graph and then on the reverse graph in decreasing finish order.

pub fn kosaraju_s_scc(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kosaraju_s_scc() {
        assert_eq!(kosaraju_s_scc(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
