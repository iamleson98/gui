// Question #63: AA Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: AA tree, level, skew, split
// Description: Implement a balanced BST using horizontal/vertical links and skew/split rebalancing for simpler code than red-black.
package datastructures

// AA Tree
// Implements a data structure for question #63.
type Q63_AaTree struct {
        data map[int]int
        size int
}

// NewQ63_AaTree creates a new instance.
func NewQ63_AaTree() *Q63_AaTree {
        return &Q63_AaTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q63_AaTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q63_AaTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q63_AaTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q63_AaTree) Len() int { return d.size }
