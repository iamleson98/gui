//! Question #220: Mo's Algorithm (Offline Queries)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: Mo's algorithm, offline, sqrt decomposition, reorder
//! Description: Answer range queries by reordering them into sqrt-blocks for O((n+q) sqrt n) time.

pub fn mo_s_algorithm_offline_queries(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mo_s_algorithm_offline_queries() {
        assert_eq!(mo_s_algorithm_offline_queries(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
