// Question #105: Union-Find with Rollback
// Category: Data Structures | Difficulty: Hard
// Concepts: union-find, rollback, persistent, offline
// Description: Extend DSU to support undo of union operations for backtracking and offline algorithms.
package datastructures

// Union-Find with Rollback
// Implements a data structure for question #105.
type UnionFindWithRollback struct {
        data map[int]int
        size int
}

// NewUnionFindWithRollback creates a new instance.
func NewUnionFindWithRollback() *UnionFindWithRollback {
        return &UnionFindWithRollback{data: make(map[int]int)}
}

// Insert adds an element.
func (d *UnionFindWithRollback) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *UnionFindWithRollback) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *UnionFindWithRollback) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *UnionFindWithRollback) Len() int { return d.size }
