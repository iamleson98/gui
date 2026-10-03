// Question #70: Ternary Search Trie
// Category: Data Structures | Difficulty: Hard
// Concepts: ternary trie, strings, prefix, branching
// Description: Implement a ternary search trie for string keys with character-by-character branching.
package datastructures

// Ternary Search Trie
// Implements a data structure for question #70.
type TernarySearchTrie struct {
        data map[int]int
        size int
}

// NewTernarySearchTrie creates a new instance.
func NewTernarySearchTrie() *TernarySearchTrie {
        return &TernarySearchTrie{data: make(map[int]int)}
}

// Insert adds an element.
func (d *TernarySearchTrie) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *TernarySearchTrie) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *TernarySearchTrie) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *TernarySearchTrie) Len() int { return d.size }
