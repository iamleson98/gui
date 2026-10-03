// Question #120: Count-Min Sketch
// Category: Data Structures | Difficulty: Hard
// Concepts: Count-Min sketch, frequency, hash functions, overestimate
// Description: Implement the Count-Min sketch for approximate frequency estimation with d hash functions.
package datastructures

// Count-Min Sketch
// Implements a data structure for question #120.
type CountMinSketch struct {
        data map[int]int
        size int
}

// NewCountMinSketch creates a new instance.
func NewCountMinSketch() *CountMinSketch {
        return &CountMinSketch{data: make(map[int]int)}
}

// Insert adds an element.
func (d *CountMinSketch) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *CountMinSketch) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *CountMinSketch) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *CountMinSketch) Len() int { return d.size }
