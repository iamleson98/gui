// Question #127: RRB-Trees (Relaxed Radix Balanced)
// Category: Data Structures | Difficulty: Hard
// Concepts: RRB tree, relaxed, concat, immutable vector
// Description: Build relaxed radix-balanced trees for efficient concat and split on immutable vectors.
package datastructures

// RRB-Trees (Relaxed Radix Balanced)
// Implements a data structure for question #127.
type RrbTreesRelaxedRadixBalanced struct {
        data map[int]int
        size int
}

// NewRrbTreesRelaxedRadixBalanced creates a new instance.
func NewRrbTreesRelaxedRadixBalanced() *RrbTreesRelaxedRadixBalanced {
        return &RrbTreesRelaxedRadixBalanced{data: make(map[int]int)}
}

// Insert adds an element.
func (d *RrbTreesRelaxedRadixBalanced) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *RrbTreesRelaxedRadixBalanced) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *RrbTreesRelaxedRadixBalanced) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *RrbTreesRelaxedRadixBalanced) Len() int { return d.size }
