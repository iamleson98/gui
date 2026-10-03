// Question #100: HyperLogLog
// Category: Data Structures | Difficulty: Hard
// Concepts: HyperLogLog, cardinality, stochastic averaging, bias correction
// Description: Implement HyperLogLog for approximate distinct-count (cardinality) estimation in fixed memory.
package datastructures

// HyperLogLog
// Implements a data structure for question #100.
type Hyperloglog struct {
        data map[int]int
        size int
}

// NewHyperloglog creates a new instance.
func NewHyperloglog() *Hyperloglog {
        return &Hyperloglog{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Hyperloglog) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Hyperloglog) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Hyperloglog) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Hyperloglog) Len() int { return d.size }
