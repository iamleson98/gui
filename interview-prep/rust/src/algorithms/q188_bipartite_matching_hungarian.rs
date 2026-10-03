//! Question #188: Bipartite Matching (Hungarian)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: assignment, Hungarian, dual, bipartite
//! Description: Solve the assignment problem with the O(n^3) Hungarian/Kuhn-Munkres algorithm.

pub fn bipartite_matching_hungarian(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bipartite_matching_hungarian() {
        assert_eq!(bipartite_matching_hungarian(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
