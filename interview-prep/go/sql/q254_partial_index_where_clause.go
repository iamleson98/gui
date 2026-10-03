// Question #254: Partial Index (WHERE Clause)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: partial index, WHERE, subset, size
// Description: Create partial indexes on a filtered subset to reduce size and speed common queries.
package sql

import "fmt"

// Partial Index (WHERE Clause)
// Implements a database design pattern for question #254.

// PartialIndexWhereClause represents the database schema/concept.
type PartialIndexWhereClause struct {
        tables map[string][]string
}

// NewPartialIndexWhereClause initializes the schema.
func NewPartialIndexWhereClause() *PartialIndexWhereClause {
        return &PartialIndexWhereClause{tables: make(map[string][]string)}
}
