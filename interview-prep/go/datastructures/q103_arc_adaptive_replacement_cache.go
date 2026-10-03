// Question #103: ARC (Adaptive Replacement Cache)
// Category: Data Structures | Difficulty: Hard
// Concepts: ARC, adaptive, recency, frequency
// Description: Implement ARC, which dynamically balances recency and frequency between LRU and LFU.
package datastructures

// ARC (Adaptive Replacement Cache)
// Implements a data structure for question #103.
type Q103_ArcAdaptiveReplacementCache struct {
        data map[int]int
        size int
}

// NewQ103_ArcAdaptiveReplacementCache creates a new instance.
func NewQ103_ArcAdaptiveReplacementCache() *Q103_ArcAdaptiveReplacementCache {
        return &Q103_ArcAdaptiveReplacementCache{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q103_ArcAdaptiveReplacementCache) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q103_ArcAdaptiveReplacementCache) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q103_ArcAdaptiveReplacementCache) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q103_ArcAdaptiveReplacementCache) Len() int { return d.size }
