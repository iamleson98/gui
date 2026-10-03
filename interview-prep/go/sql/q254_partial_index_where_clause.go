// Question #254: Partial Index (WHERE Clause)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: partial index, WHERE, subset, size
// Description: Create partial indexes on a filtered subset to reduce size and speed common queries.
package sql


// Partial Index (WHERE Clause)
// Implements a database design pattern for question #254.

// Q254_PartialIndexWhereClause represents the database schema/concept.
type Q254_PartialIndexWhereClause struct {
        tables map[string][]string
}

// NewQ254_PartialIndexWhereClause initializes the schema.
func NewQ254_PartialIndexWhereClause() *Q254_PartialIndexWhereClause {
        return &Q254_PartialIndexWhereClause{tables: make(map[string][]string)}
}
