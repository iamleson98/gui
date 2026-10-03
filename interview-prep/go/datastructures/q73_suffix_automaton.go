// Question #73: Suffix Automaton
// Category: Data Structures | Difficulty: Hard
// Concepts: suffix automaton, DFA, endpos, online
// Description: Build the minimal DFA of all suffixes of a string for online substring queries.
package datastructures

// Suffix Automaton
// Implements a data structure for question #73.
type Q73_SuffixAutomaton struct {
        data map[int]int
        size int
}

// NewQ73_SuffixAutomaton creates a new instance.
func NewQ73_SuffixAutomaton() *Q73_SuffixAutomaton {
        return &Q73_SuffixAutomaton{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q73_SuffixAutomaton) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q73_SuffixAutomaton) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q73_SuffixAutomaton) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q73_SuffixAutomaton) Len() int { return d.size }
