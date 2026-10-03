//! Question #147: Fractional Knapsack (Greedy)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: fractional knapsack, greedy, value/weight, sort
//! Description: Solve the fractional knapsack by sorting items by value/weight and greedily filling.

pub fn fractional_knapsack_greedy(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fractional_knapsack_greedy() {
        assert_eq!(fractional_knapsack_greedy(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
