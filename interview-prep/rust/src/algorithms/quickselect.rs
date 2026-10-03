//! Quickselect — find k-th smallest in expected O(n).
use rand::Rng;

pub fn quick_select(nums: &mut [i32], k: usize) -> i32 {
    if k >= nums.len() {
        panic!("k out of range");
    }
    quick_select_helper(nums, 0, nums.len() - 1, k)
}

fn quick_select_helper(nums: &mut [i32], lo: usize, hi: usize, k: usize) -> i32 {
    if lo == hi {
        return nums[lo];
    }
    let mut rng = rand::thread_rng();
    let pivot_idx = lo + rng.gen_range(0..=hi - lo);
    let pivot_idx = partition(nums, lo, hi, pivot_idx);
    if k == pivot_idx {
        nums[k]
    } else if k < pivot_idx {
        if pivot_idx == 0 {
            return nums[0];
        }
        quick_select_helper(nums, lo, pivot_idx - 1, k)
    } else {
        quick_select_helper(nums, pivot_idx + 1, hi, k)
    }
}

fn partition(nums: &mut [i32], lo: usize, hi: usize, pivot_idx: usize) -> usize {
    let pivot = nums[pivot_idx];
    nums.swap(pivot_idx, hi);
    let mut store = lo;
    for i in lo..hi {
        if nums[i] < pivot {
            nums.swap(store, i);
            store += 1;
        }
    }
    nums.swap(store, hi);
    store
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut sorted = nums.clone();
        sorted.sort();
        for k in 0..nums.len() {
            let mut arr = nums.clone();
            assert_eq!(quick_select(&mut arr, k), sorted[k]);
        }
    }
}
