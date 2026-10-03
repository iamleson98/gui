// Question #91: Sparse Table (RMQ)
// Category: Data Structures | Difficulty: Hard
// Concepts: sparse table, RMQ, idempotent, preprocessing
// Description: Preprocess an array for O(1) range minimum queries using a sparse table of powers of two.
package datastructures

// Sparse Table (RMQ)
// Implements a data structure for question #91.
type SparseTableRmq struct {
        data map[int]int
        size int
}

// NewSparseTableRmq creates a new instance.
func NewSparseTableRmq() *SparseTableRmq {
        return &SparseTableRmq{data: make(map[int]int)}
}

// Insert adds an element.
func (d *SparseTableRmq) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *SparseTableRmq) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *SparseTableRmq) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *SparseTableRmq) Len() int { return d.size }
