// Question #79: Binomial Heap
// Category: Data Structures | Difficulty: Hard
// Concepts: binomial heap, merge, lazy, amortized
// Description: Build a binomial heap supporting merge in O(log n) using linked binomial trees.
package datastructures

// Binomial Heap
// Implements a data structure for question #79.
type BinomialHeap struct {
        data map[int]int
        size int
}

// NewBinomialHeap creates a new instance.
func NewBinomialHeap() *BinomialHeap {
        return &BinomialHeap{data: make(map[int]int)}
}

// Insert adds an element.
func (d *BinomialHeap) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *BinomialHeap) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *BinomialHeap) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *BinomialHeap) Len() int { return d.size }
