package algorithms

import (
        "math/rand"
        "testing"
)

func TestLISLength(t *testing.T) {
        tests := []struct{ nums []int; want int }{
                {nil, 0},
                {[]int{1}, 1},
                {[]int{1,2,3,4,5}, 5},
                {[]int{5,4,3,2,1}, 1},
                {[]int{10,9,2,5,3,7,101,18}, 4},
                {[]int{0,1,0,3,2,3}, 4},
                {[]int{7,7,7,7,7,7,7}, 1},
        }
        for _, tt := range tests {
                got := LISLength(tt.nums)
                if got != tt.want {
                        t.Errorf("LISLength(%v) = %d, want %d", tt.nums, got, tt.want)
                }
        }
}

func TestLISReconstruct(t *testing.T) {
        nums := []int{10, 9, 2, 5, 3, 7, 101, 18}
        result := LIS(nums)
        if len(result) != 4 {
                t.Fatalf("expected length 4, got %d", len(result))
        }
        for i := 1; i < len(result); i++ {
                if result[i] <= result[i-1] {
                        t.Fatalf("not increasing: %v", result)
                }
        }
}

func TestLISRandom(t *testing.T) {
        rand.Seed(42)
        for trial := 0; trial < 100; trial++ {
                n := rand.Intn(100) + 1
                nums := make([]int, n)
                for i := range nums { nums[i] = rand.Intn(1000) }
                result := LIS(nums)
                j := 0
                for _, x := range nums {
                        if j < len(result) && x == result[j] { j++ }
                }
                if j != len(result) {
                        t.Fatalf("trial %d: result is not a subsequence", trial)
                }
        }
}

func BenchmarkLIS(b *testing.B) {
        nums := make([]int, 10000)
        for i := range nums { nums[i] = i }
        b.ReportAllocs()
        for i := 0; i < b.N; i++ {
                LISLength(nums)
        }
}
