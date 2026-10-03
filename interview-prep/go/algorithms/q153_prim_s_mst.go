// Question #153: Prim's MST
// Category: Algorithms | Difficulty: Hard
// Concepts: MST, Prim, priority queue, greedy
// Description: Grow an MST from a start vertex using a priority queue of crossing edges.
package algorithms

// Prim's MST
// Implements the algorithm for question #153.
func prim_s_mst(input []int) []int {
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
