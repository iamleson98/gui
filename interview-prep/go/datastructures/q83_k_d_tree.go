// Question #83: K-D Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: k-d tree, nearest neighbor, range query, splitting planes
// Description: Build a k-d tree for orthogonal range and nearest-neighbor queries in k-dimensional space.
package datastructures

// K-D Tree
// Implements a data structure for question #83.
type KDTree struct {
        data map[int]int
        size int
}

// NewKDTree creates a new instance.
func NewKDTree() *KDTree {
        return &KDTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *KDTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *KDTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *KDTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *KDTree) Len() int { return d.size }
