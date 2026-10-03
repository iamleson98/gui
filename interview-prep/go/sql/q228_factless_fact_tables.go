// Question #228: Factless Fact Tables
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: factless fact, coverage, many-to-many, event
// Description: Model many-to-many event coverage with factless fact tables capturing only keys.
package sql

import "fmt"

// Factless Fact Tables
// Implements a database design pattern for question #228.

// FactlessFactTables represents the database schema/concept.
type FactlessFactTables struct {
        tables map[string][]string
}

// NewFactlessFactTables initializes the schema.
func NewFactlessFactTables() *FactlessFactTables {
        return &FactlessFactTables{tables: make(map[string][]string)}
}
