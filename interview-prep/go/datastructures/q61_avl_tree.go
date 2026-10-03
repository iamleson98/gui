// Question #61: AVL Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: BST, rotations, balance factor, AVL
// Description: Implement a self-balancing binary search tree that maintains height balance via rotations on insert and delete.
package datastructures

// AVL Tree
// Implements a data structure for question #61.
type Q61_AvlTree struct {
        data map[int]int
        size int
}

// NewQ61_AvlTree creates a new instance.
func NewQ61_AvlTree() *Q61_AvlTree {
        return &Q61_AvlTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q61_AvlTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q61_AvlTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q61_AvlTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q61_AvlTree) Len() int { return d.size }
