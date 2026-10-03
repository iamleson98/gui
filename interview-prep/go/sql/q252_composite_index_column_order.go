// Question #252: Composite Index Column Order
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: composite index, column order, selectivity, range
// Description: Design composite indexes with column order matching equality, sort, and range predicates.
package sql


// Composite Index Column Order
// Implements a database design pattern for question #252.

// Q252_CompositeIndexColumnOrder represents the database schema/concept.
type Q252_CompositeIndexColumnOrder struct {
        tables map[string][]string
}

// NewQ252_CompositeIndexColumnOrder initializes the schema.
func NewQ252_CompositeIndexColumnOrder() *Q252_CompositeIndexColumnOrder {
        return &Q252_CompositeIndexColumnOrder{tables: make(map[string][]string)}
}
