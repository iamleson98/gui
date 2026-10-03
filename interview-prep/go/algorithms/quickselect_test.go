package algorithms

import (
	"math/rand"
	"sort"
	"testing"
)

func TestQuickSelect(t *testing.T) {
	nums := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	sorted := make([]int, len(nums))
	copy(sorted, nums)
	sort.Ints(sorted)
	for k := 0; k < len(nums); k++ {
		arr := make([]int, len(nums))
		copy(arr, nums)
		got := QuickSelect(arr, k)
		if got != sorted[k] {
			t.Fatalf("QuickSelect(k=%d) = %d, expected %d", k, got, sorted[k])
		}
	}
}

func TestQuickSelectRandom(t *testing.T) {
	rand.Seed(42)
	for trial := 0; trial < 100; trial++ {
		n := rand.Intn(100) + 1
		nums := rand.Perm(n)
		sorted := make([]int, n)
		copy(sorted, nums)
		sort.Ints(sorted)
		k := rand.Intn(n)
		arr := make([]int, n)
		copy(arr, nums)
		got := QuickSelect(arr, k)
		if got != sorted[k] {
			t.Fatalf("trial %d: QuickSelect(k=%d) = %d, expected %d", trial, k, got, sorted[k])
		}
	}
}

func BenchmarkQuickSelect(b *testing.B) {
	nums := rand.Perm(100000)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		arr := make([]int, len(nums))
		copy(arr, nums)
		QuickSelect(arr, 50000)
	}
}
