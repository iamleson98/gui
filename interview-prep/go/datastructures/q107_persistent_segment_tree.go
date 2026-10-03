// Question #107: Persistent Segment Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: persistent segment tree, versioning, node sharing, immutability
// Description: Build a segment tree that versions on update by sharing unchanged nodes.
package datastructures

// Persistent Segment Tree
// Implements a data structure for question #107.
type PersistentSegmentTree struct {
        data map[int]int
        size int
}

// NewPersistentSegmentTree creates a new instance.
func NewPersistentSegmentTree() *PersistentSegmentTree {
        return &PersistentSegmentTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *PersistentSegmentTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *PersistentSegmentTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *PersistentSegmentTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *PersistentSegmentTree) Len() int { return d.size }
