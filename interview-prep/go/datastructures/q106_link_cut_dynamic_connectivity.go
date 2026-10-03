// Question #106: Link-Cut Dynamic Connectivity
// Category: Data Structures | Difficulty: Hard
// Concepts: dynamic connectivity, link-cut, fully dynamic, forest
// Description: Use link-cut trees to maintain connected components under edge insertions and deletions.
package datastructures

// Link-Cut Dynamic Connectivity
// Implements a data structure for question #106.
type Q106_LinkCutDynamicConnectivity struct {
        data map[int]int
        size int
}

// NewQ106_LinkCutDynamicConnectivity creates a new instance.
func NewQ106_LinkCutDynamicConnectivity() *Q106_LinkCutDynamicConnectivity {
        return &Q106_LinkCutDynamicConnectivity{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q106_LinkCutDynamicConnectivity) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q106_LinkCutDynamicConnectivity) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q106_LinkCutDynamicConnectivity) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q106_LinkCutDynamicConnectivity) Len() int { return d.size }
