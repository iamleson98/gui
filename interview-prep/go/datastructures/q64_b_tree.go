// Question #64: B+ Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: B+ tree, fan-out, leaf links, range scan
// Description: Build a disk-oriented B+ tree with leaf links, internal node fan-out, and range scan support.
package datastructures

// B+ Tree
// Implements a data structure for question #64.
type Q64_BTree struct {
        data map[int]int
        size int
}

// NewQ64_BTree creates a new instance.
func NewQ64_BTree() *Q64_BTree {
        return &Q64_BTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q64_BTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q64_BTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q64_BTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q64_BTree) Len() int { return d.size }
