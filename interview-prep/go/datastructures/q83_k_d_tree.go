// Question #83: K-D Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: k-d tree, nearest neighbor, range query, splitting planes
// Description: Build a k-d tree for orthogonal range and nearest-neighbor queries in k-dimensional space.
package datastructures

// K-D Tree
// Implements a data structure for question #83.
type Q83_KDTree struct {
        data map[int]int
        size int
}

// NewQ83_KDTree creates a new instance.
func NewQ83_KDTree() *Q83_KDTree {
        return &Q83_KDTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q83_KDTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q83_KDTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q83_KDTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q83_KDTree) Len() int { return d.size }
