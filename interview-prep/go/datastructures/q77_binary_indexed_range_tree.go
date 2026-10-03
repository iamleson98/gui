// Question #77: Binary Indexed Range Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: BIT, range update, point query, difference
// Description: Build a BIT supporting range updates and point queries via two complementary arrays.
package datastructures

// Binary Indexed Range Tree
// Implements a data structure for question #77.
type Q77_BinaryIndexedRangeTree struct {
        data map[int]int
        size int
}

// NewQ77_BinaryIndexedRangeTree creates a new instance.
func NewQ77_BinaryIndexedRangeTree() *Q77_BinaryIndexedRangeTree {
        return &Q77_BinaryIndexedRangeTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q77_BinaryIndexedRangeTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q77_BinaryIndexedRangeTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q77_BinaryIndexedRangeTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q77_BinaryIndexedRangeTree) Len() int { return d.size }
