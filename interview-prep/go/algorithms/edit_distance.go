// Edit Distance (Levenshtein) — O(n*m) time, O(min(n,m)) space.
package algorithms

func EditDistance(s1, s2 string) int {
	n, m := len(s1), len(s2)
	if n > m {
		s1, s2 = s2, s1
		n, m = m, n
	}
	prev := make([]int, n+1)
	curr := make([]int, n+1)
	for i := 0; i <= n; i++ {
		prev[i] = i
	}
	for j := 1; j <= m; j++ {
		curr[0] = j
		for i := 1; i <= n; i++ {
			cost := 1
			if s1[i-1] == s2[j-1] {
				cost = 0
			}
			curr[i] = min3(prev[i]+1, curr[i-1]+1, prev[i-1]+cost)
		}
		prev, curr = curr, prev
	}
	return prev[n]
}

func min3(a, b, c int) int {
	m := a
	if b < m { m = b }
	if c < m { m = c }
	return m
}

func EditDistanceWithOps(s1, s2 string) []string {
	n, m := len(s1), len(s2)
	dp := make([][]int, n+1)
	for i := range dp {
		dp[i] = make([]int, m+1)
		dp[i][0] = i
	}
	for j := 0; j <= m; j++ {
		dp[0][j] = j
	}
	for i := 1; i <= n; i++ {
		for j := 1; j <= m; j++ {
			cost := 1
			if s1[i-1] == s2[j-1] { cost = 0 }
			dp[i][j] = min3(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]+cost)
		}
	}
	ops := []string{}
	i, j := n, m
	for i > 0 && j > 0 {
		cost := 1
		if s1[i-1] == s2[j-1] { cost = 0 }
		if dp[i][j] == dp[i-1][j-1]+cost && cost == 0 {
			ops = append([]string{"keep"}, ops...)
			i--; j--
		} else if dp[i][j] == dp[i-1][j-1]+1 {
			ops = append([]string{"sub"}, ops...)
			i--; j--
		} else if dp[i][j] == dp[i-1][j]+1 {
			ops = append([]string{"del"}, ops...)
			i--
		} else {
			ops = append([]string{"ins"}, ops...)
			j--
		}
	}
	for i > 0 { ops = append([]string{"del"}, ops...); i-- }
	for j > 0 { ops = append([]string{"ins"}, ops...); j-- }
	return ops
}
