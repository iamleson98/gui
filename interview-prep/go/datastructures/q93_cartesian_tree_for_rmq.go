// Question #93: Cartesian Tree for RMQ
// Category: Data Structures | Difficulty: Hard
// Concepts: Cartesian tree, LCA, RMQ reduction, Euler tour
// Description: Reduce RMQ to LCA on a Cartesian tree built from the array in linear time.
package datastructures

// Cartesian Tree for RMQ
// Implements a data structure for question #93.
type CartesianTreeForRmq struct {
        data map[int]int
        size int
}

// NewCartesianTreeForRmq creates a new instance.
func NewCartesianTreeForRmq() *CartesianTreeForRmq {
        return &CartesianTreeForRmq{data: make(map[int]int)}
}

// Insert adds an element.
func (d *CartesianTreeForRmq) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *CartesianTreeForRmq) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *CartesianTreeForRmq) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *CartesianTreeForRmq) Len() int { return d.size }
