// Question #236: Stored Procedures
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: stored procedures, encapsulation, security, maintainability
// Description: Design stored procedures for encapsulated logic and weigh security and maintainability tradeoffs.
package sql


// Stored Procedures
// Implements a database design pattern for question #236.

// Q236_StoredProcedures represents the database schema/concept.
type Q236_StoredProcedures struct {
        tables map[string][]string
}

// NewQ236_StoredProcedures initializes the schema.
func NewQ236_StoredProcedures() *Q236_StoredProcedures {
        return &Q236_StoredProcedures{tables: make(map[string][]string)}
}
