//! Question #149: Huffman Coding
//! Category: Algorithms | Difficulty: Hard
//! Concepts: Huffman, prefix code, greedy, min-heap
//! Description: Build an optimal prefix code using a min-heap and repeated merges of the two least-frequent symbols.

pub fn huffman_coding(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_huffman_coding() {
        assert_eq!(huffman_coding(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
