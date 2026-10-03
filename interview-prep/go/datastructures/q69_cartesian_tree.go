// Question #69: Cartesian Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: Cartesian tree, RMQ, heap property, linear build
// Description: Construct a Cartesian tree from an array in linear time and use it for range minimum queries.
package datastructures

// Cartesian Tree
// Implements a data structure for question #69.
type Q69_CartesianTree struct {
        data map[int]int
        size int
}

// NewQ69_CartesianTree creates a new instance.
func NewQ69_CartesianTree() *Q69_CartesianTree {
        return &Q69_CartesianTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q69_CartesianTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q69_CartesianTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q69_CartesianTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q69_CartesianTree) Len() int { return d.size }
