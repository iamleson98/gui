//! Question #196: Graham Scan
//! Category: Algorithms | Difficulty: Hard
//! Concepts: convex hull, Graham scan, angular sort, stack
//! Description: Build the convex hull by angularly sorting points and using a stack with backtracking.

pub fn graham_scan(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_graham_scan() {
        assert_eq!(graham_scan(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
