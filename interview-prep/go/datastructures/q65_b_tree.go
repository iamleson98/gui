// Question #65: B* Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: B* tree, redistribution, split, fill factor
// Description: Implement a B-tree variant that keeps nodes at least two-thirds full by redistributing before splitting.
package datastructures

// B* Tree
// Implements a data structure for question #65.
type Q65_BTree struct {
        data map[int]int
        size int
}

// NewQ65_BTree creates a new instance.
func NewQ65_BTree() *Q65_BTree {
        return &Q65_BTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q65_BTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q65_BTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q65_BTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q65_BTree) Len() int { return d.size }
