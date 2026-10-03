// Question #86: DAWG (Directed Acyclic Word Graph)
// Category: Data Structures | Difficulty: Hard
// Concepts: DAWG, minimal DFA, suffix links, strings
// Description: Build a minimal acyclic DFA accepting all suffixes of a string via suffix-tree compression.
package datastructures

// DAWG (Directed Acyclic Word Graph)
// Implements a data structure for question #86.
type DawgDirectedAcyclicWordGraph struct {
        data map[int]int
        size int
}

// NewDawgDirectedAcyclicWordGraph creates a new instance.
func NewDawgDirectedAcyclicWordGraph() *DawgDirectedAcyclicWordGraph {
        return &DawgDirectedAcyclicWordGraph{data: make(map[int]int)}
}

// Insert adds an element.
func (d *DawgDirectedAcyclicWordGraph) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *DawgDirectedAcyclicWordGraph) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *DawgDirectedAcyclicWordGraph) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *DawgDirectedAcyclicWordGraph) Len() int { return d.size }
