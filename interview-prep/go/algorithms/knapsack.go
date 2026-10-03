// 0/1 Knapsack — classic DP.
package algorithms

type Item struct {
	Weight int
	Value  int
}

func Knapsack01(items []Item, capacity int) int {
	n := len(items)
	dp := make([]int, capacity+1)
	for i := 0; i < n; i++ {
		for w := capacity; w >= items[i].Weight; w-- {
			if dp[w-items[i].Weight]+items[i].Value > dp[w] {
				dp[w] = dp[w-items[i].Weight] + items[i].Value
			}
		}
	}
	return dp[capacity]
}

func Knapsack01WithItems(items []Item, capacity int) []int {
	n := len(items)
	dp := make([][]int, n+1)
	for i := range dp {
		dp[i] = make([]int, capacity+1)
	}
	for i := 1; i <= n; i++ {
		for w := 0; w <= capacity; w++ {
			dp[i][w] = dp[i-1][w]
			if items[i-1].Weight <= w {
				if dp[i-1][w-items[i-1].Weight]+items[i-1].Value > dp[i][w] {
					dp[i][w] = dp[i-1][w-items[i-1].Weight] + items[i-1].Value
				}
			}
		}
	}
	result := []int{}
	w := capacity
	for i := n; i > 0; i-- {
		if dp[i][w] != dp[i-1][w] {
			result = append([]int{i - 1}, result...)
			w -= items[i-1].Weight
		}
	}
	return result
}
