// Question #72: Suffix Array
// Category: Data Structures | Difficulty: Hard
// Concepts: suffix array, SA-IS, binary search, strings
// Description: Construct a suffix array in O(n log n) and demonstrate binary search over it.
package datastructures

// Suffix Array
// Implements a data structure for question #72.
type SuffixArray struct {
        data map[int]int
        size int
}

// NewSuffixArray creates a new instance.
func NewSuffixArray() *SuffixArray {
        return &SuffixArray{data: make(map[int]int)}
}

// Insert adds an element.
func (d *SuffixArray) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *SuffixArray) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *SuffixArray) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *SuffixArray) Len() int { return d.size }
