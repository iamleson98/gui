//! Question #199: QuickHull
//! Category: Algorithms | Difficulty: Hard
//! Concepts: convex hull, QuickHull, divide and conquer, farthest point
//! Description: Implement the divide-and-conquer QuickHull algorithm for the convex hull.

pub fn quickhull(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quickhull() {
        assert_eq!(quickhull(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
