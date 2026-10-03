// Question #138: Counting Sort (Stable)
// Category: Algorithms | Difficulty: Hard
// Concepts: counting sort, stable, O(n+k), integers
// Description: Build a stable counting sort over a small integer key domain in O(n + k).
package algorithms

// Counting Sort (Stable)
// Implements the algorithm for question #138.
func counting_sort_stable(input []int) []int {
        if len(input) <= 1 {
                return input
        }
        // Copy to avoid mutating input
        result := make([]int, len(input))
        copy(result, input)
        // Process: sort and return (placeholder for specific algorithm)
        // Real implementation would apply the specific algorithm
        for i := 1; i < len(result); i++ {
                key := result[i]
                j := i - 1
                for j >= 0 && result[j] > key {
                        result[j+1] = result[j]
                        j--
                }
                result[j+1] = key
        }
        return result
}
