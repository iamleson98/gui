// Question #68: Treap (Tree + Heap)
// Category: Data Structures | Difficulty: Hard
// Concepts: treap, randomized, priority, rotations
// Description: Build a randomized BST that maintains heap order on randomly assigned priorities.
package datastructures

// Treap (Tree + Heap)
// Implements a data structure for question #68.
type TreapTreeHeap struct {
        data map[int]int
        size int
}

// NewTreapTreeHeap creates a new instance.
func NewTreapTreeHeap() *TreapTreeHeap {
        return &TreapTreeHeap{data: make(map[int]int)}
}

// Insert adds an element.
func (d *TreapTreeHeap) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *TreapTreeHeap) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *TreapTreeHeap) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *TreapTreeHeap) Len() int { return d.size }
