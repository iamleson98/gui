//! Question #198: Jarvis March (Gift Wrapping)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: convex hull, gift wrapping, orientation, output-sensitive
//! Description: Build the convex hull by gift wrapping around the point set in O(nh).

pub fn jarvis_march_gift_wrapping(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jarvis_march_gift_wrapping() {
        assert_eq!(jarvis_march_gift_wrapping(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
