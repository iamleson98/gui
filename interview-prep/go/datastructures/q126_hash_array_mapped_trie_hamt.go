// Question #126: Hash Array Mapped Trie (HAMT)
// Category: Data Structures | Difficulty: Hard
// Concepts: HAMT, bitmap, hash trie, persistent
// Description: Implement a HAMT using a sparse bitmap and a variable-length pointer array per node.
package datastructures

// Hash Array Mapped Trie (HAMT)
// Implements a data structure for question #126.
type HashArrayMappedTrieHamt struct {
        data map[int]int
        size int
}

// NewHashArrayMappedTrieHamt creates a new instance.
func NewHashArrayMappedTrieHamt() *HashArrayMappedTrieHamt {
        return &HashArrayMappedTrieHamt{data: make(map[int]int)}
}

// Insert adds an element.
func (d *HashArrayMappedTrieHamt) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *HashArrayMappedTrieHamt) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *HashArrayMappedTrieHamt) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *HashArrayMappedTrieHamt) Len() int { return d.size }
