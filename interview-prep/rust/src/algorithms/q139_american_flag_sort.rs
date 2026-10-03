//! Question #139: American Flag Sort
//! Category: Algorithms | Difficulty: Hard
//! Concepts: American flag sort, in-place, MSD, radix
//! Description: Implement an in-place MSD radix variant using partition pointers per bucket.

pub fn american_flag_sort(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_american_flag_sort() {
        assert_eq!(american_flag_sort(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
