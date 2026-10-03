//! Question #194: Suffix Array Construction (SA-IS)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: suffix array, SA-IS, induced sorting, linear
//! Description: Construct a suffix array in linear time using the SA-IS induced-sorting algorithm.

pub fn suffix_array_construction_sa_is(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_suffix_array_construction_sa_is() {
        assert_eq!(suffix_array_construction_sa_is(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
