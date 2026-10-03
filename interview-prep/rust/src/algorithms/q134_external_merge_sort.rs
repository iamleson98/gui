//! Question #134: External Merge Sort
//! Category: Algorithms | Difficulty: Hard
//! Concepts: external sort, k-way merge, runs, I/O
//! Description: Sort datasets larger than memory using k-way merging of sorted runs on disk.

pub fn external_merge_sort(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_external_merge_sort() {
        assert_eq!(external_merge_sort(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
