// Question #78: Binary Heap
// Category: Data Structures | Difficulty: Hard
// Concepts: binary heap, array, sift, priority queue
// Description: Implement a binary heap as an array with sift-up and sift-down for priority queue operations.
package datastructures

// Binary Heap
// Implements a data structure for question #78.
type BinaryHeap struct {
        data map[int]int
        size int
}

// NewBinaryHeap creates a new instance.
func NewBinaryHeap() *BinaryHeap {
        return &BinaryHeap{data: make(map[int]int)}
}

// Insert adds an element.
func (d *BinaryHeap) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *BinaryHeap) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *BinaryHeap) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *BinaryHeap) Len() int { return d.size }
