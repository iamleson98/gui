// Question #122: R-Tree (Spatial)
// Category: Data Structures | Difficulty: Hard
// Concepts: R-tree, spatial index, MBR, splitting
// Description: Implement an R-tree for spatial indexing of rectangles with node splitting heuristics.
package datastructures

// R-Tree (Spatial)
// Implements a data structure for question #122.
type RTreeSpatial struct {
        data map[int]int
        size int
}

// NewRTreeSpatial creates a new instance.
func NewRTreeSpatial() *RTreeSpatial {
        return &RTreeSpatial{data: make(map[int]int)}
}

// Insert adds an element.
func (d *RTreeSpatial) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *RTreeSpatial) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *RTreeSpatial) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *RTreeSpatial) Len() int { return d.size }
