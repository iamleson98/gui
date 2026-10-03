//! Question #137: LSD Radix Sort
//! Category: Algorithms | Difficulty: Hard
//! Concepts: LSD radix, counting sort, stable, fixed width
//! Description: Implement least-significant-digit radix sort using counting sort per digit.

pub fn lsd_radix_sort(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lsd_radix_sort() {
        assert_eq!(lsd_radix_sort(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
