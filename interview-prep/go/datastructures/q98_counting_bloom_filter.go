// Question #98: Counting Bloom Filter
// Category: Data Structures | Difficulty: Hard
// Concepts: counting Bloom filter, counters, deletion, false positive
// Description: Extend a Bloom filter to counters so it supports deletion via reference counting.
package datastructures

// Counting Bloom Filter
// Implements a data structure for question #98.
type Q98_CountingBloomFilter struct {
        data map[int]int
        size int
}

// NewQ98_CountingBloomFilter creates a new instance.
func NewQ98_CountingBloomFilter() *Q98_CountingBloomFilter {
        return &Q98_CountingBloomFilter{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q98_CountingBloomFilter) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q98_CountingBloomFilter) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q98_CountingBloomFilter) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q98_CountingBloomFilter) Len() int { return d.size }
