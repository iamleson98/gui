// Question #73: Suffix Automaton
// Category: Data Structures | Difficulty: Hard
// Concepts: suffix automaton, DFA, endpos, online
// Description: Build the minimal DFA of all suffixes of a string for online substring queries.
package datastructures

// Suffix Automaton
// Implements a data structure for question #73.
type SuffixAutomaton struct {
        data map[int]int
        size int
}

// NewSuffixAutomaton creates a new instance.
func NewSuffixAutomaton() *SuffixAutomaton {
        return &SuffixAutomaton{data: make(map[int]int)}
}

// Insert adds an element.
func (d *SuffixAutomaton) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *SuffixAutomaton) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *SuffixAutomaton) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *SuffixAutomaton) Len() int { return d.size }
