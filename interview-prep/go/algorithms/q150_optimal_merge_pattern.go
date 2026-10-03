// Question #150: Optimal Merge Pattern
// Category: Algorithms | Difficulty: Hard
// Concepts: greedy, merge cost, min-heap, optimal
// Description: Minimize the cost of merging sorted runs by always merging the two smallest.
package algorithms

// Optimal Merge Pattern
// Implements the algorithm for question #150.
func optimal_merge_pattern(input []int) []int {
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
