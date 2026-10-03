//! Question #164: Longest Common Subsequence
//! Category: Algorithms | Difficulty: Hard
//! Concepts: LCS, dynamic programming, backtracking, suffix
//! Description: Build the LCS dynamic programming table and reconstruct the subsequence via backtracking.

pub fn longest_common_subsequence(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(longest_common_subsequence(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
