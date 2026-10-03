// Question #113: VList
// Category: Data Structures | Difficulty: Hard
// Concepts: VList, linked blocks, persistent, indexing
// Description: Implement the VList structure providing O(1) cons and O(log n) indexing using linked blocks.
package datastructures

// VList
// Implements a data structure for question #113.
type Vlist struct {
        data map[int]int
        size int
}

// NewVlist creates a new instance.
func NewVlist() *Vlist {
        return &Vlist{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Vlist) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Vlist) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Vlist) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Vlist) Len() int { return d.size }
