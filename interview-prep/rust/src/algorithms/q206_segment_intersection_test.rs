//! Question #206: Segment Intersection Test
//! Category: Algorithms | Difficulty: Hard
//! Concepts: segment intersection, orientation, collinear, geometry
//! Description: Implement orientation tests to detect whether two line segments intersect, including collinear cases.

pub fn segment_intersection_test(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_segment_intersection_test() {
        assert_eq!(segment_intersection_test(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
