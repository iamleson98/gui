// Question #72: Suffix Array
// Category: Data Structures | Difficulty: Hard
// Concepts: suffix array, SA-IS, binary search, strings
// Description: Construct a suffix array in O(n log n) and demonstrate binary search over it.
package datastructures

// Suffix Array
// Implements a data structure for question #72.
type Q72_SuffixArray struct {
        data map[int]int
        size int
}

// NewQ72_SuffixArray creates a new instance.
func NewQ72_SuffixArray() *Q72_SuffixArray {
        return &Q72_SuffixArray{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q72_SuffixArray) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q72_SuffixArray) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q72_SuffixArray) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q72_SuffixArray) Len() int { return d.size }
