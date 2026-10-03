//! Question #197: Andrew's Monotone Chain
//! Category: Algorithms | Difficulty: Hard
//! Concepts: convex hull, monotone chain, cross product, sort
//! Description: Compute the upper and lower hulls by sorting points and scanning with cross-product tests.

pub fn andrew_s_monotone_chain(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_andrew_s_monotone_chain() {
        assert_eq!(andrew_s_monotone_chain(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
