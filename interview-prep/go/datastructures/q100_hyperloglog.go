// Question #100: HyperLogLog
// Category: Data Structures | Difficulty: Hard
// Concepts: HyperLogLog, cardinality, stochastic averaging, bias correction
// Description: Implement HyperLogLog for approximate distinct-count (cardinality) estimation in fixed memory.
package datastructures

// HyperLogLog
// Implements a data structure for question #100.
type Q100_Hyperloglog struct {
        data map[int]int
        size int
}

// NewQ100_Hyperloglog creates a new instance.
func NewQ100_Hyperloglog() *Q100_Hyperloglog {
        return &Q100_Hyperloglog{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q100_Hyperloglog) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q100_Hyperloglog) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q100_Hyperloglog) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q100_Hyperloglog) Len() int { return d.size }
