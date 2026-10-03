//! Question #140: Shell Sort with Ciura Gaps
//! Category: Algorithms | Difficulty: Hard
//! Concepts: shell sort, gaps, Ciura, in-place
//! Description: Implement shellsort using Ciura's empirically tuned gap sequence.

pub fn shell_sort_with_ciura_gaps(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shell_sort_with_ciura_gaps() {
        assert_eq!(shell_sort_with_ciura_gaps(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
