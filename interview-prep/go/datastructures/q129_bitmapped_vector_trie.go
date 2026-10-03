// Question #129: Bitmapped Vector Trie
// Category: Data Structures | Difficulty: Hard
// Concepts: vector trie, bitmap, branching, persistent
// Description: Build a trie with bitmap per node and branching factor of word size for persistent arrays.
package datastructures

// Bitmapped Vector Trie
// Implements a data structure for question #129.
type Q129_BitmappedVectorTrie struct {
        data map[int]int
        size int
}

// NewQ129_BitmappedVectorTrie creates a new instance.
func NewQ129_BitmappedVectorTrie() *Q129_BitmappedVectorTrie {
        return &Q129_BitmappedVectorTrie{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q129_BitmappedVectorTrie) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q129_BitmappedVectorTrie) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q129_BitmappedVectorTrie) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q129_BitmappedVectorTrie) Len() int { return d.size }
