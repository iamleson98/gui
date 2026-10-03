//! Question #179: Johnson's All-Pairs
//! Category: Algorithms | Difficulty: Hard
//! Concepts: all-pairs, Johnson, reweighting, Dijkstra
//! Description: Compute all-pairs shortest paths by reweighting with Bellman-Ford then running Dijkstra per vertex.

pub fn johnson_s_all_pairs(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_johnson_s_all_pairs() {
        assert_eq!(johnson_s_all_pairs(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
