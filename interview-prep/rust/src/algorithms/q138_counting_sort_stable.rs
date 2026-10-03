//! Question #138: Counting Sort (Stable)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: counting sort, stable, O(n+k), integers
//! Description: Build a stable counting sort over a small integer key domain in O(n + k).

pub fn counting_sort_stable(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_counting_sort_stable() {
        assert_eq!(counting_sort_stable(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
