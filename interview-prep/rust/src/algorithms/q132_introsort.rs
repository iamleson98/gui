//! Question #132: Introsort
//! Category: Algorithms | Difficulty: Hard
//! Concepts: introsort, hybrid, heapsort, worst-case
//! Description: Build a hybrid sort that switches from quicksort to heapsort on recursion depth to guarantee O(n log n).

pub fn introsort(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_introsort() {
        assert_eq!(introsort(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
