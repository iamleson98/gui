//! 0/1 Knapsack — classic DP.
pub struct Item {
    pub weight: usize,
    pub value: i64,
}

pub fn knapsack01(items: &[Item], capacity: usize) -> i64 {
    let n = items.len();
    let mut dp = vec![0i64; capacity + 1];
    for i in 0..n {
        for w in (items[i].weight..=capacity).rev() {
            dp[w] = dp[w].max(dp[w - items[i].weight] + items[i].value);
        }
    }
    dp[capacity]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let items = vec![
            Item { weight: 2, value: 3 },
            Item { weight: 3, value: 4 },
            Item { weight: 4, value: 5 },
            Item { weight: 5, value: 6 },
        ];
        assert_eq!(knapsack01(&items, 5), 7);
    }

    #[test]
    fn test_empty() {
        assert_eq!(knapsack01(&[], 10), 0);
    }
}
