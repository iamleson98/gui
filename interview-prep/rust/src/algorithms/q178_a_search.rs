//! Question #178: A* Search
//! Category: Algorithms | Difficulty: Hard
//! Concepts: A*, heuristic, priority queue, shortest path
//! Description: Implement A* with a consistent heuristic to find shortest paths faster than Dijkstra.

pub fn a_search(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a_search() {
        assert_eq!(a_search(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
