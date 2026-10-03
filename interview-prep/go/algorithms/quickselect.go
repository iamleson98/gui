// Quickselect — find k-th smallest in expected O(n).
package algorithms

import "math/rand"

func QuickSelect(nums []int, k int) int {
	if k < 0 || k >= len(nums) {
		panic("k out of range")
	}
	return quickSelect(nums, 0, len(nums)-1, k)
}

func quickSelect(nums []int, lo, hi, k int) int {
	if lo == hi { return nums[lo] }
	pivotIdx := lo + rand.Intn(hi-lo+1)
	pivotIdx = partition(nums, lo, hi, pivotIdx)
	if k == pivotIdx { return nums[k] }
	if k < pivotIdx { return quickSelect(nums, lo, pivotIdx-1, k) }
	return quickSelect(nums, pivotIdx+1, hi, k)
}

func partition(nums []int, lo, hi, pivotIdx int) int {
	pivot := nums[pivotIdx]
	nums[pivotIdx], nums[hi] = nums[hi], nums[pivotIdx]
	store := lo
	for i := lo; i < hi; i++ {
		if nums[i] < pivot {
			nums[store], nums[i] = nums[i], nums[store]
			store++
		}
	}
	nums[store], nums[hi] = nums[hi], nums[store]
	return store
}
