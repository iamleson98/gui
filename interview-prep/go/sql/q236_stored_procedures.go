// Question #236: Stored Procedures
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: stored procedures, encapsulation, security, maintainability
// Description: Design stored procedures for encapsulated logic and weigh security and maintainability tradeoffs.
package sql

import "fmt"

// Stored Procedures
// Implements a database design pattern for question #236.

// StoredProcedures represents the database schema/concept.
type StoredProcedures struct {
        tables map[string][]string
}

// NewStoredProcedures initializes the schema.
func NewStoredProcedures() *StoredProcedures {
        return &StoredProcedures{tables: make(map[string][]string)}
}
