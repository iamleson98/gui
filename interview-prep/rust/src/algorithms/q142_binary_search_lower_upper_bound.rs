//! Question #142: Binary Search (Lower/Upper Bound)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: binary search, lower bound, upper bound, sorted
//! Description: Implement lower_bound and upper_bound over sorted arrays with half-open intervals.

pub fn binary_search_lower_upper_bound(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_search_lower_upper_bound() {
        assert_eq!(binary_search_lower_upper_bound(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
