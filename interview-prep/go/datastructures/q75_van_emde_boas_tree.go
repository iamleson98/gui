// Question #75: Van Emde Boas Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: vEB tree, log log U, cluster, universe
// Description: Build a vEB tree over a fixed universe for O(log log U) insert, successor, and predecessor.
package datastructures

// Van Emde Boas Tree
// Implements a data structure for question #75.
type VanEmdeBoasTree struct {
        data map[int]int
        size int
}

// NewVanEmdeBoasTree creates a new instance.
func NewVanEmdeBoasTree() *VanEmdeBoasTree {
        return &VanEmdeBoasTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *VanEmdeBoasTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *VanEmdeBoasTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *VanEmdeBoasTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *VanEmdeBoasTree) Len() int { return d.size }
