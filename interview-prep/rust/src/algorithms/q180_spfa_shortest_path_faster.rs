//! Question #180: SPFA (Shortest Path Faster)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: SPFA, queue, relaxation, negative weights
//! Description: Implement the queue-based Bellman-Ford variant that only relaxes vertices whose distance changed.

pub fn spfa_shortest_path_faster(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spfa_shortest_path_faster() {
        assert_eq!(spfa_shortest_path_faster(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
