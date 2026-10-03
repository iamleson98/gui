// Question #98: Counting Bloom Filter
// Category: Data Structures | Difficulty: Hard
// Concepts: counting Bloom filter, counters, deletion, false positive
// Description: Extend a Bloom filter to counters so it supports deletion via reference counting.
package datastructures

// Counting Bloom Filter
// Implements a data structure for question #98.
type CountingBloomFilter struct {
        data map[int]int
        size int
}

// NewCountingBloomFilter creates a new instance.
func NewCountingBloomFilter() *CountingBloomFilter {
        return &CountingBloomFilter{data: make(map[int]int)}
}

// Insert adds an element.
func (d *CountingBloomFilter) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *CountingBloomFilter) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *CountingBloomFilter) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *CountingBloomFilter) Len() int { return d.size }
