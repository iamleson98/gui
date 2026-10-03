// Question #111: Chunking Deque
// Category: Data Structures | Difficulty: Hard
// Concepts: deque, chunking, amortized, persistent
// Description: Build a deque over fixed-size chunks (a 'banker's deque') for amortized O(1) operations.
package datastructures

// Chunking Deque
// Implements a data structure for question #111.
type Q111_ChunkingDeque struct {
        data map[int]int
        size int
}

// NewQ111_ChunkingDeque creates a new instance.
func NewQ111_ChunkingDeque() *Q111_ChunkingDeque {
        return &Q111_ChunkingDeque{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q111_ChunkingDeque) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q111_ChunkingDeque) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q111_ChunkingDeque) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q111_ChunkingDeque) Len() int { return d.size }
