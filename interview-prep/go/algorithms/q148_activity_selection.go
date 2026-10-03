// Question #148: Activity Selection
// Category: Algorithms | Difficulty: Hard
// Concepts: greedy, intervals, earliest finish, optimal
// Description: Solve interval scheduling by greedily picking the earliest-finishing compatible activity.
package algorithms

// Activity Selection
// Implements the algorithm for question #148.
func activity_selection(input []int) []int {
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
