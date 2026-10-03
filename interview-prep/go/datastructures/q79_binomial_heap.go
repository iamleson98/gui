// Question #79: Binomial Heap
// Category: Data Structures | Difficulty: Hard
// Concepts: binomial heap, merge, lazy, amortized
// Description: Build a binomial heap supporting merge in O(log n) using linked binomial trees.
package datastructures

// Binomial Heap
// Implements a data structure for question #79.
type Q79_BinomialHeap struct {
        data map[int]int
        size int
}

// NewQ79_BinomialHeap creates a new instance.
func NewQ79_BinomialHeap() *Q79_BinomialHeap {
        return &Q79_BinomialHeap{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q79_BinomialHeap) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q79_BinomialHeap) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q79_BinomialHeap) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q79_BinomialHeap) Len() int { return d.size }
