//! Bloom Filter — probabilistic set membership.
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct BloomFilter {
    bits: Vec<u64>,
    m: usize,
    k: usize,
}

impl BloomFilter {
    pub fn new(expected_n: usize, false_positive_rate: f64) -> Self {
        let n = expected_n.max(1) as f64;
        let p = false_positive_rate.clamp(0.0001, 0.99);
        let m = (-(n * p.ln()) / (2.0_f64.ln().powi(2))).ceil() as usize;
        let k = ((m as f64 / n) * 2.0_f64.ln()).ceil() as usize;
        Self {
            bits: vec![0; (m + 63) / 64],
            m,
            k: k.max(1),
        }
    }

    fn hash<T: Hash>(&self, item: &T, seed: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        (hasher.finish() as usize) % self.m
    }

    pub fn add<T: Hash>(&mut self, item: &T) {
        for i in 0..self.k {
            let idx = self.hash(item, i as u64);
            self.bits[idx / 64] |= 1 << (idx % 64);
        }
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.k {
            let idx = self.hash(item, i as u64);
            if self.bits[idx / 64] & (1 << (idx % 64)) == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_false_negatives() {
        let mut bf = BloomFilter::new(1000, 0.01);
        for i in 0..1000 {
            bf.add(&i);
        }
        for i in 0..1000 {
            assert!(bf.contains(&i), "false negative for {}", i);
        }
    }

    #[test]
    fn test_false_positive_rate() {
        let mut bf = BloomFilter::new(10000, 0.01);
        for i in 0..10000 {
            bf.add(&i);
        }
        let mut fp = 0;
        for i in 10000..20000 {
            if bf.contains(&i) {
                fp += 1;
            }
        }
        assert!(fp as f64 / 10000.0 < 0.05, "fp rate too high: {}", fp);
    }
}
