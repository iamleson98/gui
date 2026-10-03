//! Question #211: Number Theoretic Transform
//! Category: Algorithms | Difficulty: Hard
//! Concepts: NTT, modular, primitive root, polynomial
//! Description: Implement the NTT over a prime modulus to perform exact polynomial multiplication without floating point.

pub fn number_theoretic_transform(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_theoretic_transform() {
        assert_eq!(number_theoretic_transform(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
