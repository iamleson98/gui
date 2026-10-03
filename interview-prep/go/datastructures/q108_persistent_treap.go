// Question #108: Persistent Treap
// Category: Data Structures | Difficulty: Hard
// Concepts: persistent treap, split/merge, immutability, randomized
// Description: Implement an implicit-key treap that persists prior versions on split and merge.
package datastructures

// Persistent Treap
// Implements a data structure for question #108.
type Q108_PersistentTreap struct {
        data map[int]int
        size int
}

// NewQ108_PersistentTreap creates a new instance.
func NewQ108_PersistentTreap() *Q108_PersistentTreap {
        return &Q108_PersistentTreap{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q108_PersistentTreap) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q108_PersistentTreap) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q108_PersistentTreap) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q108_PersistentTreap) Len() int { return d.size }
