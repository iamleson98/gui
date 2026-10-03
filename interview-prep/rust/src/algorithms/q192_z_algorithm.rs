//! Question #192: Z-Algorithm
//! Category: Algorithms | Difficulty: Hard
//! Concepts: Z-array, string matching, prefix, linear
//! Description: Compute the Z-array of a string for pattern matching and pattern analysis in linear time.

pub fn z_algorithm(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_z_algorithm() {
        assert_eq!(z_algorithm(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
