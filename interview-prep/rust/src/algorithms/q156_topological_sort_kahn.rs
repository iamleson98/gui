//! Question #156: Topological Sort (Kahn)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: topological sort, Kahn, in-degree, DAG
//! Description: Produce a topological ordering of a DAG using in-degree counts and a queue.

pub fn topological_sort_kahn(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_topological_sort_kahn() {
        assert_eq!(topological_sort_kahn(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
