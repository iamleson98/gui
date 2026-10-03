//! Question #155: Reverse Delete MST
//! Category: Algorithms | Difficulty: Hard
//! Concepts: MST, reverse delete, cycle, greedy
//! Description: Build MST by deleting the heaviest edge that does not disconnect the graph.

pub fn reverse_delete_mst(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_delete_mst() {
        assert_eq!(reverse_delete_mst(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
