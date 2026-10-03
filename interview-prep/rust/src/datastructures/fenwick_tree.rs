//! Fenwick Tree (Binary Indexed Tree) — prefix-sum updates/queries in O(log n).
pub struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self { tree: vec![0; n + 1], n }
    }

    pub fn from_arr(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut ft = Self { tree: vec![0; n + 1], n };
        for i in 0..n {
            ft.tree[i + 1] = arr[i];
        }
        for i in 1..=n {
            let p = i + (i & i.wrapping_neg());
            if p <= n {
                ft.tree[p] += ft.tree[i];
            }
        }
        ft
    }

    pub fn update(&mut self, mut i: usize, delta: i64) {
        i += 1; // 1-indexed
        while i <= self.n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    pub fn query(&self, mut i: i64) -> i64 {
        i += 1; // 1-indexed
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i as usize];
            i -= i & i.wrapping_neg();
        }
        sum
    }

    pub fn query_range(&self, l: usize, r: usize) -> i64 {
        if l > r {
            return 0;
        }
        self.query(r as i64) - if l == 0 { 0 } else { self.query(l as i64 - 1) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let arr = vec![1, 3, 5, 7, 9, 11];
        let ft = FenwickTree::from_arr(&arr);
        assert_eq!(ft.query(2), 9);
        assert_eq!(ft.query_range(1, 3), 15);
    }

    #[test]
    fn test_update() {
        let arr = vec![10, 20, 30, 40, 50];
        let mut ft = FenwickTree::from_arr(&arr);
        ft.update(2, 5);
        assert_eq!(ft.query(4), 155);
    }
}
