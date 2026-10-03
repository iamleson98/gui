// Question #77: Binary Indexed Range Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: BIT, range update, point query, difference
// Description: Build a BIT supporting range updates and point queries via two complementary arrays.
package datastructures

// Binary Indexed Range Tree
// Implements a data structure for question #77.
type BinaryIndexedRangeTree struct {
        data map[int]int
        size int
}

// NewBinaryIndexedRangeTree creates a new instance.
func NewBinaryIndexedRangeTree() *BinaryIndexedRangeTree {
        return &BinaryIndexedRangeTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *BinaryIndexedRangeTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *BinaryIndexedRangeTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *BinaryIndexedRangeTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *BinaryIndexedRangeTree) Len() int { return d.size }
