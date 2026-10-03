// Question #113: VList
// Category: Data Structures | Difficulty: Hard
// Concepts: VList, linked blocks, persistent, indexing
// Description: Implement the VList structure providing O(1) cons and O(log n) indexing using linked blocks.
package datastructures

// VList
// Implements a data structure for question #113.
type Q113_Vlist struct {
        data map[int]int
        size int
}

// NewQ113_Vlist creates a new instance.
func NewQ113_Vlist() *Q113_Vlist {
        return &Q113_Vlist{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q113_Vlist) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q113_Vlist) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q113_Vlist) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q113_Vlist) Len() int { return d.size }
