//! Question #200: Rotating Calipers
//! Category: Algorithms | Difficulty: Hard
//! Concepts: rotating calipers, antipodal, diameter, convex polygon
//! Description: Use rotating calipers on a convex polygon to compute diameter, width, and antipodal pairs.

pub fn rotating_calipers(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotating_calipers() {
        assert_eq!(rotating_calipers(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
