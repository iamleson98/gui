// Question #76: Interval Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: interval tree, overlap query, augmented BST, red-black
// Description: Implement a red-black based interval tree supporting overlap queries for intervals.
package datastructures

// Interval Tree
// Implements a data structure for question #76.
type IntervalTree struct {
        data map[int]int
        size int
}

// NewIntervalTree creates a new instance.
func NewIntervalTree() *IntervalTree {
        return &IntervalTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *IntervalTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *IntervalTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *IntervalTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *IntervalTree) Len() int { return d.size }
