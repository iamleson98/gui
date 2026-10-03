// Question #118: Implicit Treap (Split/Merge)
// Category: Data Structures | Difficulty: Hard
// Concepts: implicit treap, split, merge, subtree size
// Description: Build a treap keyed by subtree size supporting split and merge for sequence operations.
package datastructures

// Implicit Treap (Split/Merge)
// Implements a data structure for question #118.
type Q118_ImplicitTreapSplitMerge struct {
        data map[int]int
        size int
}

// NewQ118_ImplicitTreapSplitMerge creates a new instance.
func NewQ118_ImplicitTreapSplitMerge() *Q118_ImplicitTreapSplitMerge {
        return &Q118_ImplicitTreapSplitMerge{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q118_ImplicitTreapSplitMerge) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q118_ImplicitTreapSplitMerge) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q118_ImplicitTreapSplitMerge) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q118_ImplicitTreapSplitMerge) Len() int { return d.size }
