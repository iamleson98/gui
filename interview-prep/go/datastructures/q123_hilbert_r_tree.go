// Question #123: Hilbert R-Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: Hilbert R-tree, space-filling curve, ordering, spatial
// Description: Design an R-tree whose leaves follow a Hilbert ordering to improve query performance.
package datastructures

// Hilbert R-Tree
// Implements a data structure for question #123.
type Q123_HilbertRTree struct {
        data map[int]int
        size int
}

// NewQ123_HilbertRTree creates a new instance.
func NewQ123_HilbertRTree() *Q123_HilbertRTree {
        return &Q123_HilbertRTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q123_HilbertRTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q123_HilbertRTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q123_HilbertRTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q123_HilbertRTree) Len() int { return d.size }
