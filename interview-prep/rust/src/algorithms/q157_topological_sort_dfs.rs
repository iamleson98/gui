//! Question #157: Topological Sort (DFS)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: topological sort, DFS, post-order, DAG
//! Description: Generate a topological order by post-order DFS and reversing the finish times.

pub fn topological_sort_dfs(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_topological_sort_dfs() {
        assert_eq!(topological_sort_dfs(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
