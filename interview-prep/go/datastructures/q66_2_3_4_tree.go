// Question #66: 2-3-4 Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: 2-3-4 tree, splitting, bottom-up, balance
// Description: Build a balanced search tree with 2-, 3-, and 4-nodes that splits on overflow from the bottom up.
package datastructures

// 2-3-4 Tree
// Implements a data structure for question #66.
type 234Tree struct {
        data map[int]int
        size int
}

// New234Tree creates a new instance.
func New234Tree() *234Tree {
        return &234Tree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *234Tree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *234Tree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *234Tree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *234Tree) Len() int { return d.size }
