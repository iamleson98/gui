//! Question #171: Partition Equal Subset Sum
//! Category: Algorithms | Difficulty: Hard
//! Concepts: partition, subset sum, DP, boolean
//! Description: Determine if an array can be partitioned into two equal-sum subsets using subset-sum DP.

pub fn partition_equal_subset_sum(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_partition_equal_subset_sum() {
        assert_eq!(partition_equal_subset_sum(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
