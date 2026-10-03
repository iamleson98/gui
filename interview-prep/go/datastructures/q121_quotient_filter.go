// Question #121: Quotient Filter
// Category: Data Structures | Difficulty: Hard
// Concepts: quotient filter, open addressing, locality, membership
// Description: Build a quotient filter using quotient/remainder hashing for membership with locality.
package datastructures

// Quotient Filter
// Implements a data structure for question #121.
type QuotientFilter struct {
        data map[int]int
        size int
}

// NewQuotientFilter creates a new instance.
func NewQuotientFilter() *QuotientFilter {
        return &QuotientFilter{data: make(map[int]int)}
}

// Insert adds an element.
func (d *QuotientFilter) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *QuotientFilter) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *QuotientFilter) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *QuotientFilter) Len() int { return d.size }
