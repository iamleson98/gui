// Manacher's Algorithm — longest palindromic substring in O(n).
package algorithms

func LongestPalindrome(s string) string {
	if len(s) == 0 { return "" }
	t := "^"
	for i := 0; i < len(s); i++ {
		t += "#" + string(s[i])
	}
	t += "#$"
	n := len(t)
	p := make([]int, n)
	c, r := 0, 0
	maxLen, center := 0, 0
	for i := 1; i < n-1; i++ {
		mirror := 2*c - i
		if r > i {
			p[i] = minInt(r-i, p[mirror])
		}
		for t[i+p[i]+1] == t[i-p[i]-1] {
			p[i]++
		}
		if i+p[i] > r {
			c = i
			r = i + p[i]
		}
		if p[i] > maxLen {
			maxLen = p[i]
			center = i
		}
	}
	start := (center - maxLen) / 2
	return s[start : start+maxLen]
}

func minInt(a, b int) int {
	if a < b { return a }
	return b
}
