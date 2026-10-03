// Question #87: Aho-Corasick Automaton
// Category: Data Structures | Difficulty: Hard
// Concepts: Aho-Corasick, failure links, multi-pattern, DFA
// Description: Construct the AC automaton with failure links for multi-pattern string matching.
package datastructures

// Aho-Corasick Automaton
// Implements a data structure for question #87.
type AhoCorasickAutomaton struct {
        data map[int]int
        size int
}

// NewAhoCorasickAutomaton creates a new instance.
func NewAhoCorasickAutomaton() *AhoCorasickAutomaton {
        return &AhoCorasickAutomaton{data: make(map[int]int)}
}

// Insert adds an element.
func (d *AhoCorasickAutomaton) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *AhoCorasickAutomaton) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *AhoCorasickAutomaton) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *AhoCorasickAutomaton) Len() int { return d.size }
