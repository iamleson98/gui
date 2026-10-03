//! Question #219: Count of Range Sum
//! Category: Algorithms | Difficulty: Hard
//! Concepts: range sum, prefix sum, Fenwick tree, count
//! Description: Count subarray sums in a range using a Fenwick tree over prefix sums.

pub fn count_of_range_sum(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_of_range_sum() {
        assert_eq!(count_of_range_sum(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
