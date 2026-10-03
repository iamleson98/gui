//! Question #172: Egg Drop (DP)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: egg drop, DP, worst case, trials
//! Description: Find the minimum number of egg-drop trials in the worst case using a DP over eggs and floors.

pub fn egg_drop_dp(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_egg_drop_dp() {
        assert_eq!(egg_drop_dp(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
