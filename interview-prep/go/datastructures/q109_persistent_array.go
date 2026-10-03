// Question #109: Persistent Array
// Category: Data Structures | Difficulty: Hard
// Concepts: persistent array, fat node, versioning, immutability
// Description: Build a persistent array using a fat-node or balanced-tree representation with O(log n) updates.
package datastructures

// Persistent Array
// Implements a data structure for question #109.
type PersistentArray struct {
        data map[int]int
        size int
}

// NewPersistentArray creates a new instance.
func NewPersistentArray() *PersistentArray {
        return &PersistentArray{data: make(map[int]int)}
}

// Insert adds an element.
func (d *PersistentArray) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *PersistentArray) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *PersistentArray) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *PersistentArray) Len() int { return d.size }
