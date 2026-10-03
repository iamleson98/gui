// Question #241: Recursive CTEs
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: recursive CTE, hierarchy, traversal, anchor
// Description: Model hierarchies and graph traversals with recursive CTEs using an anchor and a recursive member.
package sql


// Recursive CTEs
// Implements a database design pattern for question #241.

// Q241_RecursiveCtes represents the database schema/concept.
type Q241_RecursiveCtes struct {
        tables map[string][]string
}

// NewQ241_RecursiveCtes initializes the schema.
func NewQ241_RecursiveCtes() *Q241_RecursiveCtes {
        return &Q241_RecursiveCtes{tables: make(map[string][]string)}
}
