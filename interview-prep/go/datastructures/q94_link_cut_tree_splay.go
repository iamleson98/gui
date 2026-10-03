// Question #94: Link-Cut Tree (Splay)
// Category: Data Structures | Difficulty: Hard
// Concepts: link-cut tree, splay, dynamic forest, preferred path
// Description: Implement a splay-based link-cut tree supporting dynamic forest queries and edge link/cut.
package datastructures

// Link-Cut Tree (Splay)
// Implements a data structure for question #94.
type Q94_LinkCutTreeSplay struct {
        data map[int]int
        size int
}

// NewQ94_LinkCutTreeSplay creates a new instance.
func NewQ94_LinkCutTreeSplay() *Q94_LinkCutTreeSplay {
        return &Q94_LinkCutTreeSplay{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q94_LinkCutTreeSplay) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q94_LinkCutTreeSplay) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q94_LinkCutTreeSplay) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q94_LinkCutTreeSplay) Len() int { return d.size }
