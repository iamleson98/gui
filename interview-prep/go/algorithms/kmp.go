// KMP String Matching — O(n+m).
package algorithms

func ComputeFailureFunction(pattern string) []int {
	n := len(pattern)
	if n == 0 { return nil }
	fail := make([]int, n)
	j := 0
	for i := 1; i < n; {
		if pattern[i] == pattern[j] {
			j++
			fail[i] = j
			i++
		} else if j > 0 {
			j = fail[j-1]
		} else {
			fail[i] = 0
			i++
		}
	}
	return fail
}

func KMPSearch(text, pattern string) []int {
	if len(pattern) == 0 { return []int{0} }
	if len(pattern) > len(text) { return nil }
	fail := ComputeFailureFunction(pattern)
	result := []int{}
	j := 0
	for i := 0; i < len(text); {
		if text[i] == pattern[j] {
			i++
			j++
			if j == len(pattern) {
				result = append(result, i-j)
				j = fail[j-1]
			}
		} else if j > 0 {
			j = fail[j-1]
		} else {
			i++
		}
	}
	return result
}
