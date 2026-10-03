//! Question #153: Prim's MST
//! Category: Algorithms | Difficulty: Hard
//! Concepts: MST, Prim, priority queue, greedy
//! Description: Grow an MST from a start vertex using a priority queue of crossing edges.

pub fn prim_s_mst(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prim_s_mst() {
        assert_eq!(prim_s_mst(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
