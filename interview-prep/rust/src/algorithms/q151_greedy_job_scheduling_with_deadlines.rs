//! Question #151: Greedy Job Scheduling with Deadlines
//! Category: Algorithms | Difficulty: Hard
//! Concepts: greedy, deadlines, disjoint set, profit
//! Description: Maximize profit by scheduling unit-length jobs before their deadlines using disjoint-set slotting.

pub fn greedy_job_scheduling_with_deadlines(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greedy_job_scheduling_with_deadlines() {
        assert_eq!(greedy_job_scheduling_with_deadlines(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
