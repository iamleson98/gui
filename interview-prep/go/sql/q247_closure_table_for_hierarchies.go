// Question #247: Closure Table for Hierarchies
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: closure table, hierarchy, ancestor, descendant
// Description: Store all ancestor-descendant pairs in a closure table for fast descendant and depth queries.
package sql

import "fmt"

// Closure Table for Hierarchies
// Implements a database design pattern for question #247.

// ClosureTableForHierarchies represents the database schema/concept.
type ClosureTableForHierarchies struct {
        tables map[string][]string
}

// NewClosureTableForHierarchies initializes the schema.
func NewClosureTableForHierarchies() *ClosureTableForHierarchies {
        return &ClosureTableForHierarchies{tables: make(map[string][]string)}
}
