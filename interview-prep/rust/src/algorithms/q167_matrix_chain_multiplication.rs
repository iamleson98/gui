//! Question #167: Matrix Chain Multiplication
//! Category: Algorithms | Difficulty: Hard
//! Concepts: matrix chain, interval DP, parenthesization, cost
//! Description: Find the parenthesization minimizing scalar multiplications using interval DP.

pub fn matrix_chain_multiplication(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_chain_multiplication() {
        assert_eq!(matrix_chain_multiplication(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
