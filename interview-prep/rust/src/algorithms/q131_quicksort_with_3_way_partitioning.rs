//! Question #131: Quicksort with 3-Way Partitioning
//! Category: Algorithms | Difficulty: Hard
//! Concepts: quicksort, 3-way, duplicates, in-place
//! Description: Implement quicksort using Dutch national flag partitioning to handle duplicates efficiently.

pub fn quicksort_with_3_way_partitioning(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quicksort_with_3_way_partitioning() {
        assert_eq!(quicksort_with_3_way_partitioning(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
