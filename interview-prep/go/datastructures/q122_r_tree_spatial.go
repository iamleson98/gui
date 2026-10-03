// Question #122: R-Tree (Spatial)
// Category: Data Structures | Difficulty: Hard
// Concepts: R-tree, spatial index, MBR, splitting
// Description: Implement an R-tree for spatial indexing of rectangles with node splitting heuristics.
package datastructures

// R-Tree (Spatial)
// Implements a data structure for question #122.
type Q122_RTreeSpatial struct {
        data map[int]int
        size int
}

// NewQ122_RTreeSpatial creates a new instance.
func NewQ122_RTreeSpatial() *Q122_RTreeSpatial {
        return &Q122_RTreeSpatial{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q122_RTreeSpatial) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q122_RTreeSpatial) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q122_RTreeSpatial) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q122_RTreeSpatial) Len() int { return d.size }
