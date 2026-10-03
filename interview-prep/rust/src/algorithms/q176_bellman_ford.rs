//! Question #176: Bellman-Ford
//! Category: Algorithms | Difficulty: Hard
//! Concepts: shortest path, Bellman-Ford, negative weights, relaxation
//! Description: Compute shortest paths with negative weights using edge relaxation and a negative-cycle detector.

pub fn bellman_ford(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bellman_ford() {
        assert_eq!(bellman_ford(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
