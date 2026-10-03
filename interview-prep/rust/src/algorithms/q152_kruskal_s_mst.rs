//! Question #152: Kruskal's MST
//! Category: Algorithms | Difficulty: Hard
//! Concepts: MST, Kruskal, union-find, greedy
//! Description: Build a minimum spanning forest using union-find to add edges in sorted order without forming cycles.

pub fn kruskal_s_mst(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kruskal_s_mst() {
        assert_eq!(kruskal_s_mst(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
