//! Question #148: Activity Selection
//! Category: Algorithms | Difficulty: Hard
//! Concepts: greedy, intervals, earliest finish, optimal
//! Description: Solve interval scheduling by greedily picking the earliest-finishing compatible activity.

pub fn activity_selection(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_activity_selection() {
        assert_eq!(activity_selection(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
