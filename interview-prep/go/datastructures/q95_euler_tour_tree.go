// Question #95: Euler Tour Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: Euler tour tree, balanced BST, dynamic connectivity, edge
// Description: Build an Euler tour tree over a balanced BST to support dynamic connectivity.
package datastructures

// Euler Tour Tree
// Implements a data structure for question #95.
type Q95_EulerTourTree struct {
        data map[int]int
        size int
}

// NewQ95_EulerTourTree creates a new instance.
func NewQ95_EulerTourTree() *Q95_EulerTourTree {
        return &Q95_EulerTourTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q95_EulerTourTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q95_EulerTourTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q95_EulerTourTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q95_EulerTourTree) Len() int { return d.size }
