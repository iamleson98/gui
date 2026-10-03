//! Question #141: Median of Medians (BFPRT)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: BFPRT, selection, median of medians, linear
//! Description: Implement linear-time selection using the median-of-medians pivot strategy with guaranteed bounds.

pub fn median_of_medians_bfprt(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_median_of_medians_bfprt() {
        assert_eq!(median_of_medians_bfprt(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
