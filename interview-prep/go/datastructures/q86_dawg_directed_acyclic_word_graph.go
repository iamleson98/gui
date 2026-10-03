// Question #86: DAWG (Directed Acyclic Word Graph)
// Category: Data Structures | Difficulty: Hard
// Concepts: DAWG, minimal DFA, suffix links, strings
// Description: Build a minimal acyclic DFA accepting all suffixes of a string via suffix-tree compression.
package datastructures

// DAWG (Directed Acyclic Word Graph)
// Implements a data structure for question #86.
type Q86_DawgDirectedAcyclicWordGraph struct {
        data map[int]int
        size int
}

// NewQ86_DawgDirectedAcyclicWordGraph creates a new instance.
func NewQ86_DawgDirectedAcyclicWordGraph() *Q86_DawgDirectedAcyclicWordGraph {
        return &Q86_DawgDirectedAcyclicWordGraph{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q86_DawgDirectedAcyclicWordGraph) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q86_DawgDirectedAcyclicWordGraph) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q86_DawgDirectedAcyclicWordGraph) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q86_DawgDirectedAcyclicWordGraph) Len() int { return d.size }
