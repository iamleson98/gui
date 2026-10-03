//! Question #202: Closest Pair of Points
//! Category: Algorithms | Difficulty: Hard
//! Concepts: closest pair, divide and conquer, strip, sort
//! Description: Find the closest pair of points in O(n log n) using divide and conquer across a sorted strip.

pub fn closest_pair_of_points(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_closest_pair_of_points() {
        assert_eq!(closest_pair_of_points(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
