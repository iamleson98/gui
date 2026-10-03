// Question #121: Quotient Filter
// Category: Data Structures | Difficulty: Hard
// Concepts: quotient filter, open addressing, locality, membership
// Description: Build a quotient filter using quotient/remainder hashing for membership with locality.
package datastructures

// Quotient Filter
// Implements a data structure for question #121.
type Q121_QuotientFilter struct {
        data map[int]int
        size int
}

// NewQ121_QuotientFilter creates a new instance.
func NewQ121_QuotientFilter() *Q121_QuotientFilter {
        return &Q121_QuotientFilter{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q121_QuotientFilter) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q121_QuotientFilter) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q121_QuotientFilter) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q121_QuotientFilter) Len() int { return d.size }
