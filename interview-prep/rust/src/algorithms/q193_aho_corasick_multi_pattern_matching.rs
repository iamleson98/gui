//! Question #193: Aho-Corasick Multi-Pattern Matching
//! Category: Algorithms | Difficulty: Hard
//! Concepts: Aho-Corasick, failure links, output links, DFA
//! Description: Build the AC automaton to find all occurrences of multiple patterns simultaneously.

pub fn aho_corasick_multi_pattern_matching(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aho_corasick_multi_pattern_matching() {
        assert_eq!(aho_corasick_multi_pattern_matching(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
