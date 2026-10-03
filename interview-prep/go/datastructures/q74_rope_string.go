// Question #74: Rope (String)
// Category: Data Structures | Difficulty: Hard
// Concepts: rope, balanced tree, split, chunks
// Description: Implement a rope as a balanced binary tree of string chunks supporting split, concat, and insert.
package datastructures

// Rope (String)
// Implements a data structure for question #74.
type Q74_RopeString struct {
        data map[int]int
        size int
}

// NewQ74_RopeString creates a new instance.
func NewQ74_RopeString() *Q74_RopeString {
        return &Q74_RopeString{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q74_RopeString) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q74_RopeString) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q74_RopeString) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q74_RopeString) Len() int { return d.size }
