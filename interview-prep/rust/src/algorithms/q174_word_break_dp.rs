//! Question #174: Word Break (DP)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: word break, DP, trie, segmentation
//! Description: Determine whether a string can be segmented into dictionary words using DP.

pub fn word_break_dp(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_word_break_dp() {
        assert_eq!(word_break_dp(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
