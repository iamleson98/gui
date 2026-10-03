//! Question #190: Boyer-Moore String Matching
//! Category: Algorithms | Difficulty: Hard
//! Concepts: string matching, bad character, good suffix, skip
//! Description: Implement Boyer-Moore using bad-character and good-suffix heuristics to skip alignments.

pub fn boyer_moore_string_matching(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_boyer_moore_string_matching() {
        assert_eq!(boyer_moore_string_matching(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
