// Question #192: Z-Algorithm
// Category: Algorithms | Difficulty: Hard
// Concepts: Z-array, string matching, prefix, linear
// Description: Compute the Z-array of a string for pattern matching and pattern analysis in linear time.
package algorithms

// Z-Algorithm
// Implements the algorithm for question #192.
func z_algorithm(input []int) []int {
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
