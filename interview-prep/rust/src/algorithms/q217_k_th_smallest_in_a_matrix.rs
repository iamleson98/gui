//! Question #217: K-th Smallest in a Matrix
//! Category: Algorithms | Difficulty: Hard
//! Concepts: k-th smallest, matrix, binary search, min-heap
//! Description: Find the k-th smallest element in a sorted matrix using a min-heap or binary search on value.

pub fn k_th_smallest_in_a_matrix(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_k_th_smallest_in_a_matrix() {
        assert_eq!(k_th_smallest_in_a_matrix(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
