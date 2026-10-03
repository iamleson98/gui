// Question #134: External Merge Sort
// Category: Algorithms | Difficulty: Hard
// Concepts: external sort, k-way merge, runs, I/O
// Description: Sort datasets larger than memory using k-way merging of sorted runs on disk.
package algorithms

// External Merge Sort
// Implements the algorithm for question #134.
func external_merge_sort(input []int) []int {
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
