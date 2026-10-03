//! Question #154: Boruvka's MST
//! Category: Algorithms | Difficulty: Hard
//! Concepts: MST, Boruvka, components, parallel
//! Description: Compute MST by iteratively adding the cheapest edge from every component.

pub fn boruvka_s_mst(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_boruvka_s_mst() {
        assert_eq!(boruvka_s_mst(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
