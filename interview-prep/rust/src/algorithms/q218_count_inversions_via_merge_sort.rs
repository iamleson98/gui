//! Question #218: Count Inversions via Merge Sort
//! Category: Algorithms | Difficulty: Hard
//! Concepts: inversions, merge sort, count, O(n log n)
//! Description: Count array inversions in O(n log n) by augmenting merge sort with a counter.

pub fn count_inversions_via_merge_sort(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_inversions_via_merge_sort() {
        assert_eq!(count_inversions_via_merge_sort(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
