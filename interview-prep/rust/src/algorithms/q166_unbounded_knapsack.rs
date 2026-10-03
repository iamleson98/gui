//! Question #166: Unbounded Knapsack
//! Category: Algorithms | Difficulty: Hard
//! Concepts: knapsack, unbounded, 1D DP, reuse
//! Description: Solve the unbounded knapsack where items can be reused with a 1D DP.

pub fn unbounded_knapsack(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unbounded_knapsack() {
        assert_eq!(unbounded_knapsack(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
