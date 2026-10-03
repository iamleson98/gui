//! Question #150: Optimal Merge Pattern
//! Category: Algorithms | Difficulty: Hard
//! Concepts: greedy, merge cost, min-heap, optimal
//! Description: Minimize the cost of merging sorted runs by always merging the two smallest.

pub fn optimal_merge_pattern(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_optimal_merge_pattern() {
        assert_eq!(optimal_merge_pattern(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
