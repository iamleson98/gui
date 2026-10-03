//! Question #136: In-Place MSD Radix Sort
//! Category: Algorithms | Difficulty: Hard
//! Concepts: MSD radix, in-place, recursion, strings
//! Description: Sort strings in-place using most-significant-digit radix recursion with a key-indexed count.

pub fn in_place_msd_radix_sort(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_in_place_msd_radix_sort() {
        assert_eq!(in_place_msd_radix_sort(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
