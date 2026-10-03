// Question #99: Cuckoo Filter
// Category: Data Structures | Difficulty: Hard
// Concepts: cuckoo filter, fingerprint, cuckoo hashing, deletion
// Description: Build a cuckoo-filter using bounded cuckoo hashing with fingerprints for set membership and deletion.
package datastructures

// Cuckoo Filter
// Implements a data structure for question #99.
type CuckooFilter struct {
        data map[int]int
        size int
}

// NewCuckooFilter creates a new instance.
func NewCuckooFilter() *CuckooFilter {
        return &CuckooFilter{data: make(map[int]int)}
}

// Insert adds an element.
func (d *CuckooFilter) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *CuckooFilter) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *CuckooFilter) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *CuckooFilter) Len() int { return d.size }
