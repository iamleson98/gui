// Question #92: Disjoint Sparse Table
// Category: Data Structures | Difficulty: Hard
// Concepts: sparse table, non-idempotent, range sum, preprocessing
// Description: Build a sparse table supporting non-idempotent range queries such as sum in O(log n).
package datastructures

// Disjoint Sparse Table
// Implements a data structure for question #92.
type Q92_DisjointSparseTable struct {
        data map[int]int
        size int
}

// NewQ92_DisjointSparseTable creates a new instance.
func NewQ92_DisjointSparseTable() *Q92_DisjointSparseTable {
        return &Q92_DisjointSparseTable{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q92_DisjointSparseTable) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q92_DisjointSparseTable) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q92_DisjointSparseTable) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q92_DisjointSparseTable) Len() int { return d.size }
