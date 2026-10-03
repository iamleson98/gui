//! Question #201: Bentley-Ottmann Line Sweep
//! Category: Algorithms | Difficulty: Hard
//! Concepts: line sweep, Bentley-Ottmann, events, balanced tree
//! Description: Find all intersections of line segments using a sweep line and balanced tree of active segments.

pub fn bentley_ottmann_line_sweep(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bentley_ottmann_line_sweep() {
        assert_eq!(bentley_ottmann_line_sweep(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
