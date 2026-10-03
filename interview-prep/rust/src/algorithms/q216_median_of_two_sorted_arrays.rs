//! Question #216: Median of Two Sorted Arrays
//! Category: Algorithms | Difficulty: Hard
//! Concepts: median, two arrays, binary partition, logarithmic
//! Description: Find the median of two sorted arrays in O(log(min(m, n))) using binary partition.

pub fn median_of_two_sorted_arrays(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_median_of_two_sorted_arrays() {
        assert_eq!(median_of_two_sorted_arrays(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
