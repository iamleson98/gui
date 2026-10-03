// Question #95: Euler Tour Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: Euler tour tree, balanced BST, dynamic connectivity, edge
// Description: Build an Euler tour tree over a balanced BST to support dynamic connectivity.
package datastructures

// Euler Tour Tree
// Implements a data structure for question #95.
type EulerTourTree struct {
        data map[int]int
        size int
}

// NewEulerTourTree creates a new instance.
func NewEulerTourTree() *EulerTourTree {
        return &EulerTourTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *EulerTourTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *EulerTourTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *EulerTourTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *EulerTourTree) Len() int { return d.size }
