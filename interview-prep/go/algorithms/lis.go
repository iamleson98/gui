// Longest Increasing Subsequence — O(n log n) using patience sorting.
package algorithms

func LISLength(nums []int) int {
	if len(nums) == 0 { return 0 }
	tails := []int{}
	for _, x := range nums {
		lo, hi := 0, len(tails)
		for lo < hi {
			mid := (lo + hi) / 2
			if tails[mid] < x { lo = mid + 1 } else { hi = mid }
		}
		if lo == len(tails) {
			tails = append(tails, x)
		} else {
			tails[lo] = x
		}
	}
	return len(tails)
}

func LIS(nums []int) []int {
	if len(nums) == 0 { return nil }
	tails := []int{}
	tailsIdx := []int{}
	prev := make([]int, len(nums))
	for i := range prev { prev[i] = -1 }
	for i, x := range nums {
		lo, hi := 0, len(tails)
		for lo < hi {
			mid := (lo + hi) / 2
			if tails[mid] < x { lo = mid + 1 } else { hi = mid }
		}
		if lo == len(tails) {
			tails = append(tails, x)
			tailsIdx = append(tailsIdx, i)
		} else {
			tails[lo] = x
			tailsIdx[lo] = i
		}
		if lo > 0 { prev[i] = tailsIdx[lo-1] }
	}
	result := []int{}
	k := tailsIdx[len(tailsIdx)-1]
	for k >= 0 {
		result = append([]int{nums[k]}, result...)
		k = prev[k]
	}
	return result
}
