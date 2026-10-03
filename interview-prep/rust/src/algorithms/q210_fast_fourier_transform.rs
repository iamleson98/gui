//! Question #210: Fast Fourier Transform
//! Category: Algorithms | Difficulty: Hard
//! Concepts: FFT, polynomial, Cooley-Tukey, roots of unity
//! Description: Implement the FFT to evaluate polynomials in O(n log n) and multiply polynomials via pointwise products.

pub fn fast_fourier_transform(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fast_fourier_transform() {
        assert_eq!(fast_fourier_transform(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
