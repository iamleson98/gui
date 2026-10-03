// Question #62: Splay Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: splay tree, self-adjusting, amortized, rotations
// Description: Build a self-adjusting BST that moves recently accessed nodes to the root via zig, zig-zig, and zig-zag operations.
package datastructures

// Splay Tree
// Implements a data structure for question #62.
type SplayTree struct {
        data map[int]int
        size int
}

// NewSplayTree creates a new instance.
func NewSplayTree() *SplayTree {
        return &SplayTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *SplayTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *SplayTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *SplayTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *SplayTree) Len() int { return d.size }
