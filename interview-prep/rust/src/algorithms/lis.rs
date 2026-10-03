//! Longest Increasing Subsequence — O(n log n).
pub fn lis_length(nums: &[i32]) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut tails: Vec<i32> = Vec::new();
    for &x in nums {
        let pos = tails.binary_search(&x).unwrap_or_else(|p| p);
        if pos == tails.len() {
            tails.push(x);
        } else {
            tails[pos] = x;
        }
    }
    tails.len()
}

pub fn lis(nums: &[i32]) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }
    let mut tails: Vec<i32> = Vec::new();
    let mut tails_idx: Vec<usize> = Vec::new();
    let mut prev: Vec<Option<usize>> = vec![None; nums.len()];
    for (i, &x) in nums.iter().enumerate() {
        let pos = tails.binary_search(&x).unwrap_or_else(|p| p);
        if pos == tails.len() {
            tails.push(x);
            tails_idx.push(i);
        } else {
            tails[pos] = x;
            tails_idx[pos] = i;
        }
        if pos > 0 {
            prev[i] = Some(tails_idx[pos - 1]);
        }
    }
    let mut result = Vec::new();
    let mut k = tails_idx[tails_idx.len() - 1];
    while k != usize::MAX {
        result.push(nums[k]);
        k = match prev[k] {
            Some(p) => p,
            None => break,
        };
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lis_length() {
        assert_eq!(lis_length(&[]), 0);
        assert_eq!(lis_length(&[1]), 1);
        assert_eq!(lis_length(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(lis_length(&[5, 4, 3, 2, 1]), 1);
        assert_eq!(lis_length(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}
