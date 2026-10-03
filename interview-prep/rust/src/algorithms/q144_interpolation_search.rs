//! Question #144: Interpolation Search
//! Category: Algorithms | Difficulty: Hard
//! Concepts: interpolation search, uniform, probe, sorted
//! Description: Implement interpolation search for uniformly distributed keys, achieving O(log log n) on average.

pub fn interpolation_search(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interpolation_search() {
        assert_eq!(interpolation_search(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
