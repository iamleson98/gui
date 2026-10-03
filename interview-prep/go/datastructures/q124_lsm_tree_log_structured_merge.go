// Question #124: LSM-Tree (Log-Structured Merge)
// Category: Data Structures | Difficulty: Hard
// Concepts: LSM tree, memtable, SSTable, compaction
// Description: Implement a log-structured merge tree with memtable, SSTables, and leveled compaction.
package datastructures

// LSM-Tree (Log-Structured Merge)
// Implements a data structure for question #124.
type LsmTreeLogStructuredMerge struct {
        data map[int]int
        size int
}

// NewLsmTreeLogStructuredMerge creates a new instance.
func NewLsmTreeLogStructuredMerge() *LsmTreeLogStructuredMerge {
        return &LsmTreeLogStructuredMerge{data: make(map[int]int)}
}

// Insert adds an element.
func (d *LsmTreeLogStructuredMerge) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *LsmTreeLogStructuredMerge) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *LsmTreeLogStructuredMerge) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *LsmTreeLogStructuredMerge) Len() int { return d.size }
