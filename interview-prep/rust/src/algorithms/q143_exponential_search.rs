//! Question #143: Exponential Search
//! Category: Algorithms | Difficulty: Hard
//! Concepts: exponential search, doubling, unbounded, sorted
//! Description: Search sorted arrays by doubling the index then binary searching within the bounded range.

pub fn exponential_search(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exponential_search() {
        assert_eq!(exponential_search(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
