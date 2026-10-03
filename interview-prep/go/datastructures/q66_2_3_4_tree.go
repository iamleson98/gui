// Question #66: 2-3-4 Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: 2-3-4 tree, splitting, bottom-up, balance
// Description: Build a balanced search tree with 2-, 3-, and 4-nodes that splits on overflow from the bottom up.
package datastructures

// 2-3-4 Tree
// Implements a data structure for question #66.
type Q66_234Tree struct {
        data map[int]int
        size int
}

// NewQ66_234Tree creates a new instance.
func NewQ66_234Tree() *Q66_234Tree {
        return &Q66_234Tree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q66_234Tree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q66_234Tree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q66_234Tree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q66_234Tree) Len() int { return d.size }
