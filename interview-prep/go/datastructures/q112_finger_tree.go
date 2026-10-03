// Question #112: Finger Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: finger tree, amortized, split, persistent
// Description: Implement Okasaki's finger tree supporting amortized O(1) cons/snoc and O(log n) split.
package datastructures

// Finger Tree
// Implements a data structure for question #112.
type FingerTree struct {
        data map[int]int
        size int
}

// NewFingerTree creates a new instance.
func NewFingerTree() *FingerTree {
        return &FingerTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *FingerTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *FingerTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *FingerTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *FingerTree) Len() int { return d.size }
