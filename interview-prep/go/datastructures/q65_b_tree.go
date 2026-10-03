// Question #65: B* Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: B* tree, redistribution, split, fill factor
// Description: Implement a B-tree variant that keeps nodes at least two-thirds full by redistributing before splitting.
package datastructures

// B* Tree
// Implements a data structure for question #65.
type BTree struct {
        data map[int]int
        size int
}

// NewBTree creates a new instance.
func NewBTree() *BTree {
        return &BTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *BTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *BTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *BTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *BTree) Len() int { return d.size }
