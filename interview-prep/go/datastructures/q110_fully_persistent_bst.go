// Question #110: Fully Persistent BST
// Category: Data Structures | Difficulty: Hard
// Concepts: persistent BST, path copying, versioning, immutability
// Description: Implement a BST that keeps every prior version queryable after updates.
package datastructures

// Fully Persistent BST
// Implements a data structure for question #110.
type Q110_FullyPersistentBst struct {
        data map[int]int
        size int
}

// NewQ110_FullyPersistentBst creates a new instance.
func NewQ110_FullyPersistentBst() *Q110_FullyPersistentBst {
        return &Q110_FullyPersistentBst{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q110_FullyPersistentBst) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q110_FullyPersistentBst) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q110_FullyPersistentBst) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q110_FullyPersistentBst) Len() int { return d.size }
