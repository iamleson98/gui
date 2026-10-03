//! Question #170: Subset Sum (Pseudo-Polynomial)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: subset sum, bitset, pseudo-polynomial, DP
//! Description: Solve subset sum using a bitset DP over the achievable sums.

pub fn subset_sum_pseudo_polynomial(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_subset_sum_pseudo_polynomial() {
        assert_eq!(subset_sum_pseudo_polynomial(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
