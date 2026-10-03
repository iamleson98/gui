// Question #228: Factless Fact Tables
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: factless fact, coverage, many-to-many, event
// Description: Model many-to-many event coverage with factless fact tables capturing only keys.
package sql


// Factless Fact Tables
// Implements a database design pattern for question #228.

// Q228_FactlessFactTables represents the database schema/concept.
type Q228_FactlessFactTables struct {
        tables map[string][]string
}

// NewQ228_FactlessFactTables initializes the schema.
func NewQ228_FactlessFactTables() *Q228_FactlessFactTables {
        return &Q228_FactlessFactTables{tables: make(map[string][]string)}
}
