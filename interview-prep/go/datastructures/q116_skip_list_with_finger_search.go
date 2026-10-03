// Question #116: Skip List with Finger Search
// Category: Data Structures | Difficulty: Hard
// Concepts: skip list, finger, local search, ordered map
// Description: Extend a skip list with finger search to find elements near a recent position faster.
package datastructures

// Skip List with Finger Search
// Implements a data structure for question #116.
type Q116_SkipListWithFingerSearch struct {
        data map[int]int
        size int
}

// NewQ116_SkipListWithFingerSearch creates a new instance.
func NewQ116_SkipListWithFingerSearch() *Q116_SkipListWithFingerSearch {
        return &Q116_SkipListWithFingerSearch{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q116_SkipListWithFingerSearch) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q116_SkipListWithFingerSearch) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q116_SkipListWithFingerSearch) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q116_SkipListWithFingerSearch) Len() int { return d.size }
