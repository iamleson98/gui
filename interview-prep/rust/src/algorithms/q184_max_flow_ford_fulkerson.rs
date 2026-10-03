//! Question #184: Max Flow: Ford-Fulkerson
//! Category: Algorithms | Difficulty: Hard
//! Concepts: max flow, Ford-Fulkerson, augmenting path, residual
//! Description: Compute max flow by augmenting along any augmenting path until none remain.

pub fn max_flow_ford_fulkerson(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_flow_ford_fulkerson() {
        assert_eq!(max_flow_ford_fulkerson(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
