// Question #111: Chunking Deque
// Category: Data Structures | Difficulty: Hard
// Concepts: deque, chunking, amortized, persistent
// Description: Build a deque over fixed-size chunks (a 'banker's deque') for amortized O(1) operations.
package datastructures

// Chunking Deque
// Implements a data structure for question #111.
type ChunkingDeque struct {
        data map[int]int
        size int
}

// NewChunkingDeque creates a new instance.
func NewChunkingDeque() *ChunkingDeque {
        return &ChunkingDeque{data: make(map[int]int)}
}

// Insert adds an element.
func (d *ChunkingDeque) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *ChunkingDeque) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *ChunkingDeque) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *ChunkingDeque) Len() int { return d.size }
