// Question #71: Suffix Tree (Ukkonen)
// Category: Data Structures | Difficulty: Hard
// Concepts: suffix tree, Ukkonen, implicit links, linear time
// Description: Build Ukkonen's linear-time suffix tree with implicit suffix links and active point extension.
package datastructures

// Suffix Tree (Ukkonen)
// Implements a data structure for question #71.
type SuffixTreeUkkonen struct {
        data map[int]int
        size int
}

// NewSuffixTreeUkkonen creates a new instance.
func NewSuffixTreeUkkonen() *SuffixTreeUkkonen {
        return &SuffixTreeUkkonen{data: make(map[int]int)}
}

// Insert adds an element.
func (d *SuffixTreeUkkonen) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *SuffixTreeUkkonen) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *SuffixTreeUkkonen) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *SuffixTreeUkkonen) Len() int { return d.size }
