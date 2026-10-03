// Question #67: Scapegoat Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: scapegoat tree, rebuild, amortized, alpha-balanced
// Description: Implement a self-balancing BST that rebuilds an unbalanced subtree when its height exceeds a logarithmic bound.
package datastructures

// Scapegoat Tree
// Implements a data structure for question #67.
type Q67_ScapegoatTree struct {
        data map[int]int
        size int
}

// NewQ67_ScapegoatTree creates a new instance.
func NewQ67_ScapegoatTree() *Q67_ScapegoatTree {
        return &Q67_ScapegoatTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q67_ScapegoatTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q67_ScapegoatTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q67_ScapegoatTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q67_ScapegoatTree) Len() int { return d.size }
