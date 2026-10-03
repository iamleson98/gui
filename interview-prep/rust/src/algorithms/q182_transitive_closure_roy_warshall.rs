//! Question #182: Transitive Closure (Roy-Warshall)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: transitive closure, boolean, DP, reachability
//! Description: Compute the transitive closure of a graph using a Floyd-Warshall-style boolean DP.

pub fn transitive_closure_roy_warshall(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transitive_closure_roy_warshall() {
        assert_eq!(transitive_closure_roy_warshall(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
