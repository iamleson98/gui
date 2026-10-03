// Question #252: Composite Index Column Order
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: composite index, column order, selectivity, range
// Description: Design composite indexes with column order matching equality, sort, and range predicates.
package sql

import "fmt"

// Composite Index Column Order
// Implements a database design pattern for question #252.

// CompositeIndexColumnOrder represents the database schema/concept.
type CompositeIndexColumnOrder struct {
        tables map[string][]string
}

// NewCompositeIndexColumnOrder initializes the schema.
func NewCompositeIndexColumnOrder() *CompositeIndexColumnOrder {
        return &CompositeIndexColumnOrder{tables: make(map[string][]string)}
}
