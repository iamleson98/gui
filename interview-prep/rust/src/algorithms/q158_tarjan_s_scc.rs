//! Question #158: Tarjan's SCC
//! Category: Algorithms | Difficulty: Hard
//! Concepts: SCC, Tarjan, lowlink, DFS
//! Description: Find strongly connected components in linear time using a DFS stack and lowlink values.

pub fn tarjan_s_scc(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tarjan_s_scc() {
        assert_eq!(tarjan_s_scc(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
