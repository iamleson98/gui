// Question #241: Recursive CTEs
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: recursive CTE, hierarchy, traversal, anchor
// Description: Model hierarchies and graph traversals with recursive CTEs using an anchor and a recursive member.
package sql

import "fmt"

// Recursive CTEs
// Implements a database design pattern for question #241.

// RecursiveCtes represents the database schema/concept.
type RecursiveCtes struct {
        tables map[string][]string
}

// NewRecursiveCtes initializes the schema.
func NewRecursiveCtes() *RecursiveCtes {
        return &RecursiveCtes{tables: make(map[string][]string)}
}
