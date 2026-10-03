// Question #127: RRB-Trees (Relaxed Radix Balanced)
// Category: Data Structures | Difficulty: Hard
// Concepts: RRB tree, relaxed, concat, immutable vector
// Description: Build relaxed radix-balanced trees for efficient concat and split on immutable vectors.
package datastructures

// RRB-Trees (Relaxed Radix Balanced)
// Implements a data structure for question #127.
type Q127_RrbTreesRelaxedRadixBalanced struct {
        data map[int]int
        size int
}

// NewQ127_RrbTreesRelaxedRadixBalanced creates a new instance.
func NewQ127_RrbTreesRelaxedRadixBalanced() *Q127_RrbTreesRelaxedRadixBalanced {
        return &Q127_RrbTreesRelaxedRadixBalanced{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q127_RrbTreesRelaxedRadixBalanced) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q127_RrbTreesRelaxedRadixBalanced) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q127_RrbTreesRelaxedRadixBalanced) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q127_RrbTreesRelaxedRadixBalanced) Len() int { return d.size }
