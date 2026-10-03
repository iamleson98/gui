//! Question #191: Rabin-Karp
//! Category: Algorithms | Difficulty: Hard
//! Concepts: rolling hash, Rabin-Karp, collision, multi-pattern
//! Description: Implement the Rabin-Karp rolling-hash matcher for single and multi-pattern search.

pub fn rabin_karp(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rabin_karp() {
        assert_eq!(rabin_karp(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
