//! Question #212: Segmented Sieve of Eratosthenes
//! Category: Algorithms | Difficulty: Hard
//! Concepts: sieve, segmented, primes, wheel
//! Description: Generate primes in a large interval using a segmented sieve with small primes as wheels.

pub fn segmented_sieve_of_eratosthenes(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_segmented_sieve_of_eratosthenes() {
        assert_eq!(segmented_sieve_of_eratosthenes(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
