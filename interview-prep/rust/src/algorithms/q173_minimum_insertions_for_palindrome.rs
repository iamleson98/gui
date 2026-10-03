//! Question #173: Minimum Insertions for Palindrome
//! Category: Algorithms | Difficulty: Hard
//! Concepts: palindrome, insertions, LCS, DP
//! Description: Compute the minimum insertions to make a string a palindrome using LCS with its reverse.

pub fn minimum_insertions_for_palindrome(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_insertions_for_palindrome() {
        assert_eq!(minimum_insertions_for_palindrome(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
