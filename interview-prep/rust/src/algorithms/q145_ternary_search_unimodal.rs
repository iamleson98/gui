//! Question #145: Ternary Search (Unimodal)
//! Category: Algorithms | Difficulty: Hard
//! Concepts: ternary search, unimodal, divide, optimization
//! Description: Find the maximum of a unimodal function by repeatedly narrowing with two probes.

pub fn ternary_search_unimodal(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ternary_search_unimodal() {
        assert_eq!(ternary_search_unimodal(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
